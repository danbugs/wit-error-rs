# `wit-error-rs`

This is the closest I could get to an user-experience similar to the `thiserror` Rust crate.

## How To Use

In your `Cargo.toml`, add:

```
wit-error-rs = { git = "https://github.com/danbugs/wit-error-rs", rev = "05362f1a4a3a9dc6a1de39195e06d2d5d6491a5e" }
```

If, in your wit, you have type that you want to classify as an Error type (i.e., should implement `std::error::Error`), like:
```
// file error.wit
variant error {
    error-with-description(string)
}
```

You can use `wit-error-rs` to classify it that way, like so:
```rs
// wit_bindgen_rust::export!("<path>/<to>/error.wit"); <- assumed
wit_error-rs::impl_error!(error::Error);
```

In addition, you can convert from things like `anyhow::Error` to your own `error::Error`, like so:
```rs
// wit_bindgen_rust::export!("<path>/<to>/error.wit"); <- assumed
wit_error-rs::impl_from!(anyhow::Error, error::Error::ErrorWithDescription);
```

For a more detailed explanation and usage, refer to the examples directory.