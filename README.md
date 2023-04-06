# Httpbin in Rust

## :Status: Alpha

This is a reimplementation of _httpbin_ for two purposes:

1. To demonstrate (and test) the abilities of an http library for rust
2. To make a static binary (1.6MB) providing all the httpbin functionality

(not affiliated to the original httpbin)

## Installation (of the binary)

```shell
cargo install httpbin
```

Or use it as a library

* http: http://github.com/swindon-rs/tk-http
* httpbin: http://httpbin.org

By default listens on port 8080. Many endpoints of the original httpbin are currently unimplemented.

## License

Licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](http://opensource.org/licenses/MIT)
  at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
