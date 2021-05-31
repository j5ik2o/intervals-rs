# intervals-rs

A Rust crate for intervals.

[![Workflow Status](https://github.com/j5ik2o/intervals-rs/workflows/Rust/badge.svg)](https://github.com/j5ik2o/intervals-rs/actions?query=workflow%3A%22Rust%22)
[![crates.io](https://img.shields.io/crates/v/intervals-rs.svg)](https://crates.io/crates/intervals-rs)
[![docs.rs](https://docs.rs/intervals-rs/badge.svg)](https://docs.rs/intervals-rs)
[![dependency status](https://deps.rs/repo/github/j5ik2o/intervals-rs/status.svg)](https://deps.rs/repo/github/j5ik2o/intervals-rs)
[![tokei](https://tokei.rs/b1/github/j5ik2o/intervals-rs)](https://github.com/XAMPPRocky/tokei)

## Install to Cargo.toml

Add this to your `Cargo.toml`:

```toml
[dependencies]
intervals-rs = "<<version>>"
```

## Usage

```rust
let range = Interval::closed(
  LimitValue::Limit(Decimal::from_f32(-5.5).unwrap()),
  LimitValue::Limit(Decimal::from_f32(6.6).unwrap()),
);
assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(5.0).unwrap())));
assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(-5.5).unwrap())));
assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(-5.4999).unwrap())));
assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(6.6).unwrap())));
assert!(!range.includes(&LimitValue::Limit(Decimal::from_f32(6.601).unwrap())));
assert!(!range.includes(&LimitValue::Limit(Decimal::from_f32(-5.501).unwrap())));
```



## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
