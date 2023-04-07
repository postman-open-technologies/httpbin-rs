# Httpbin in Rust

## :Status: Alpha

![Icecream emoji logo](https://raw.githubusercontent.com/postman-open-technologies/httpbin-rs/main/static/apple-touch-icon.png)
<img src="https://raw.githubusercontent.com/postman-open-technologies/httpbin-rs/main/static/crab_emoji.svg" alt="Rust crab logo/emoji" width="250px" height="250px" />

This is a reimplementation of `httpbin` for two purposes:

1. To demonstrate (and test) the abilities of an http library for rust
2. To make a static binary (1.6MB) providing all the httpbin functionality

(not affiliated to the original httpbin)

## Installation (of the binary)

```shell
cargo install httpbin
```

Or use it as a library

* `http`: http://github.com/swindon-rs/tk-http
* `httpbin`: http://httpbin.org

By default listens on 0.0.0.0:8080, but you can pass a port number on the command-line. Many endpoints of the original httpbin are currently unimplemented.

## TODO

* Add all unimplemented functionality
* Redirect `/docs` to [OpenAPI docs](https://redocly.github.io/redoc/?url=https://raw.githubusercontent.com/postman-open-technologies/httpbin-rs/main/src/templates/openapi.yaml)

## Attribution

Originally written by [Paul Colomiets](https://githubcom/tailhook). Based on [`httpbin`](https://github.com/postmanlabs/httpbin) by [Kenneth Reitz](https://github.com/kennethreitz).

## License

Licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](http://opensource.org/licenses/MIT)
  at your option.

This project also contains code from the original `httpbin` which is [ISC licensed](http://opensource.org/licenses/ISC)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
