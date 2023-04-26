use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rand::distributions::{Distribution, WeightedIndex};
use serde_json::json;
use std::str::FromStr;

const ASCII_ART: &str = r#"
    -=[ teapot ]=-

       _...._
     .'  _ _ `.
    | ."` ^ `". _,
    \_;`"---"`|//
      |       ;/
      \_     _/
        `\"\"\"`
"#;

const REDIRECT_LOCATION: &str = "/redirect/1";

const ACCEPTED_MEDIA_TYPES: &[&str] = &[
    "image/webp",
    "image/svg+xml",
    "image/jpeg",
    "image/png",
    "image/*",
];

pub fn routes() -> Router {
    Router::new().route(
        "/status/:code",
        get(status)
            .post(status)
            .put(status)
            .patch(status)
            .delete(status)
            .trace(status)
            .head(status),
    )
}

async fn status(Path(code): Path<String>) -> impl IntoResponse {
    let invalid_status_code = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header(header::CONTENT_TYPE, mime::TEXT_PLAIN.as_ref())
        .body(Body::from("Invalid status code"));

    let code = if code.contains(',') {
        let codes = code.split(',');
        let mut rng = rand::thread_rng();
        let mut choices: Vec<&str> = vec![];
        let mut weights: Vec<f32> = vec![];
        for code in codes {
            if let Some((code, weight)) = code.split_once(':') {
                choices.push(code);
                weights.push(weight.parse::<f32>().unwrap_or(1.0));
            } else {
                choices.push(code);
                weights.push(1.0);
            }
            if StatusCode::from_str(choices.last().unwrap()).is_err() {
                return invalid_status_code.unwrap();
            }
        }
        let dist = WeightedIndex::new(&weights).unwrap();
        choices[dist.sample(&mut rng)]
    } else {
        &code
    };

    match StatusCode::from_str(code) {
        Ok(code) => {
            let builder = Response::builder().status(code);
            let mut body = Body::empty();
            match code {
                StatusCode::MOVED_PERMANENTLY
                | StatusCode::FOUND
                | StatusCode::SEE_OTHER
                | StatusCode::NOT_MODIFIED
                | StatusCode::TEMPORARY_REDIRECT => builder
                    .header(header::LOCATION, REDIRECT_LOCATION)
                    .body(body)
                    .unwrap(),
                StatusCode::UNAUTHORIZED => builder
                    .header(header::WWW_AUTHENTICATE, "Basic realm=\"Fake realm\"")
                    .body(body)
                    .unwrap(),
                StatusCode::PAYMENT_REQUIRED => {
                    body = Body::from("Show me the money!");
                    builder
                        .header("x-more-info", "https://youtu.be/FFrag8ll85w")
                        .header(header::CONTENT_TYPE, mime::TEXT_PLAIN.as_ref())
                        .body(body)
                        .unwrap()
                }
                StatusCode::NOT_ACCEPTABLE => {
                    body = Body::from(
                        json!({
                          "message": "Client did not request a supported media type.",
                          "accept": ACCEPTED_MEDIA_TYPES
                        })
                        .to_string(),
                    );
                    builder
                        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string())
                        .body(body)
                        .unwrap()
                }
                StatusCode::PROXY_AUTHENTICATION_REQUIRED => builder
                    .header(header::PROXY_AUTHENTICATE, "Basic realm=\"Fake realm\"")
                    .body(body)
                    .unwrap(),
                StatusCode::IM_A_TEAPOT => builder
                    .header("x-more-info", "http://tools.ietf.org/html/rfc2324")
                    .header(header::CONTENT_TYPE, mime::TEXT_PLAIN.as_ref())
                    .body(Body::from(ASCII_ART))
                    .unwrap(),
                _ => builder.body(body).unwrap(),
            }
        }
        Err(_) => invalid_status_code.unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{header, HeaderValue, Method, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn selects_a_single_status_code() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status/200")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn supports_multiple_http_methods() {
        let app = routes();

        let methods = vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::TRACE,
            Method::HEAD,
        ];

        for method in methods {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(method)
                        .uri("/status/200")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn chooses_a_random_status_code_when_multiple_provided() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status/200,201,202,204")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let choices = vec![
            StatusCode::OK,
            StatusCode::CREATED,
            StatusCode::ACCEPTED,
            StatusCode::NO_CONTENT,
        ];

        assert!(choices.contains(&response.status()));
    }

    #[tokio::test]
    async fn chooses_a_random_status_code_when_multiple_provided_with_weights() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status/200:0.1,201:0.1,202:0.1,204:1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let choices = vec![
            StatusCode::OK,
            StatusCode::CREATED,
            StatusCode::ACCEPTED,
            StatusCode::NO_CONTENT,
        ];

        assert!(choices.contains(&response.status()));
    }

    #[tokio::test]
    async fn chooses_a_higher_weighted_random_status_code_more_often() {
        let app = routes();
        let mut ok_returns: u16 = 0;
        let mut created_returns: u16 = 0;
        let mut accepted_returns: u16 = 0;
        let mut no_content_returns: u16 = 0;

        for _num in 0..1000 {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/status/200:0.1,201:0.25,202:0.75,204:1")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            match response.status() {
                StatusCode::OK => ok_returns += 1,
                StatusCode::CREATED => created_returns += 1,
                StatusCode::ACCEPTED => accepted_returns += 1,
                StatusCode::NO_CONTENT => no_content_returns += 1,
                _ => {}
            }
        }

        assert!(no_content_returns > ok_returns);
        assert!(no_content_returns > created_returns);
        assert!(no_content_returns > accepted_returns);
        assert!(accepted_returns > created_returns);
        assert!(accepted_returns > ok_returns);
        assert!(created_returns > ok_returns);
    }

    #[tokio::test]
    async fn returns_an_error_on_invalid_status_code_by_itself() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status/jalapeno")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::TEXT_PLAIN.as_ref()))
        );
    }

    #[tokio::test]
    async fn returns_an_error_on_invalid_status_code_among_multiple() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status/200,201,204,jalapeno")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::TEXT_PLAIN.as_ref()))
        );
    }

    #[tokio::test]
    async fn redirects_have_location_header() {
        let app = routes();

        let redirects = vec![
            StatusCode::MOVED_PERMANENTLY,
            StatusCode::FOUND,
            StatusCode::SEE_OTHER,
            StatusCode::NOT_MODIFIED,
            StatusCode::TEMPORARY_REDIRECT,
        ];

        for redirect in redirects {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(format!("/status/{}", redirect.as_str()))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), redirect);
            assert!(response.headers().get(header::LOCATION).is_some());
        }
    }

    #[tokio::test]
    async fn unauthorized_has_www_authenticate_header() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status/401")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert!(response.headers().get(header::WWW_AUTHENTICATE).is_some());
    }

    #[tokio::test]
    async fn payment_required_has_text_body() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status/402")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::PAYMENT_REQUIRED);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::TEXT_PLAIN.as_ref()))
        );
    }

    #[tokio::test]
    async fn not_acceptable_has_json_body() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status/406")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()))
        );
    }

    #[tokio::test]
    async fn proxy_authentication_required_has_proxy_authenticate_header() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status/407")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::PROXY_AUTHENTICATION_REQUIRED);
        assert!(response.headers().get(header::PROXY_AUTHENTICATE).is_some());
    }
    #[tokio::test]
    async fn teapot_has_body() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/status/418")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::IM_A_TEAPOT);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::TEXT_PLAIN.as_ref()))
        );

        let body = response.collect().await.unwrap().to_bytes();
        assert!(std::str::from_utf8(&body).is_ok())
    }
}
