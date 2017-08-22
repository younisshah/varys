# varys

A simple `crate` to shorten URLs using http://is.gd/.

## Installation

In your `Cargo.toml` file add under `[dependencies]` section

```ini
[dependencies]
varys = "0.1.0"
```

## Shorten a URI

```rust
let shortened_url: String = varys::shorten("https://www.rust-lang.org/");
```

## Tests

There are no tests.


## License

MIT