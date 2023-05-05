# Httpbin in Rust

## Status: Alpha

![Icecream emoji logo](https://raw.githubusercontent.com/postman-open-technologies/httpbin-rs/main/static/apple-touch-icon.png)
<img src="https://raw.githubusercontent.com/postman-open-technologies/httpbin-rs/main/static/crab_emoji.svg" alt="Rust crab logo/emoji" width="250px" height="250px" />

This is a reimplementation of `httpbin` for two purposes:

1. To demonstrate (and test) the abilities of an http library for rust
2. To make a static binary (4.6MB) providing all the httpbin functionality

(not affiliated to the original httpbin)

By default listens on 0.0.0.0:8080, but you can pass a port number on the command-line. Many endpoints of the original httpbin are currently unimplemented.

## Implementation Status

- [ ] HTTP Methods
- [ ] Auth
- [x] Status codes
- [x] Request inspection
- [ ] Response inspection
- [ ] Response formats
- [ ] Dynamic data
- [ ] Cookies
- [ ] Images
- [ ] Redirects
- [ ] Anything

## Attribution

Originally written by [Paul Colomiets](https://githubcom/tailhook). Based on [`httpbin`](https://github.com/postmanlabs/httpbin) by [Kenneth Reitz](https://github.com/kennethreitz).

## License

Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

This project also contains code from the original `httpbin` which is [ISC licensed](http://opensource.org/licenses/ISC)
