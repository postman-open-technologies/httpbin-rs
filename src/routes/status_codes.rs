use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rand::distributions::{Distribution, WeightedIndex};
use serde_json::json;
use std::str::FromStr;

const ASCII_ART: &'static str = r#"
    -=[ teapot ]=-

       _...._
     .'  _ _ `.
    | ."` ^ `". _,
    \_;`"---"`|//
      |       ;/
      \_     _/
        `\"\"\"`
"#;

const REDIRECT_LOCATION: &'static str = "/redirect/1";

const ACCEPTED_MEDIA_TYPES: &'static [&'static str] = &[
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
            .delete(status),
    )
}

async fn status(Path(code): Path<String>) -> impl IntoResponse {
    let code = if code.contains(",") {
        let codes = code.split(",");
        let mut rng = rand::thread_rng();
        let mut choices: Vec<&str> = vec![];
        let mut weights: Vec<f32> = vec![];
        for code in codes {
            if let Some((code, weight)) = code.split_once(":") {
                choices.push(code);
                weights.push(weight.parse::<f32>().unwrap_or(1.0));
            } else {
                choices.push(code);
                weights.push(1.0);
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
            let mut body = "".to_string();
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
                    body = "Fuck you, pay me!".to_string(); // 👀 https://github.com/postmanlabs/httpbin/blob/master/httpbin/helpers.py#L221
                    builder
                        .header("x-more-info", "http://vimeo.com/22053820")
                        .body(body)
                        .unwrap()
                }
                StatusCode::NOT_ACCEPTABLE => {
                    body = json!({
                      "message": "Client did not request a supported media type.",
                      "accept": ACCEPTED_MEDIA_TYPES
                    })
                    .to_string();
                    builder
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(body)
                        .unwrap()
                }
                StatusCode::PROXY_AUTHENTICATION_REQUIRED => builder
                    .header(header::PROXY_AUTHENTICATE, "Basic realm=\"Fake realm\"")
                    .body(body)
                    .unwrap(),
                StatusCode::IM_A_TEAPOT => builder
                    .header("x-more-info", "http://tools.ietf.org/html/rfc2324")
                    .body(ASCII_ART.to_string())
                    .unwrap(),
                _ => builder.body(body).unwrap(),
            }
        }
        Err(_) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Invalid status code.".to_string())
            .unwrap(),
    }
}
