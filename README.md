# `bytes-pool`

Allocation-free sharing of bytes and strings using [`Bytes`](https://docs.rs/bytes/latest/bytes/struct.Bytes.html) from the [`bytes`](https://docs.rs/bytes) crate as storage.

[![crates.io](https://img.shields.io/crates/v/bytes-pool.svg)](https://crates.io/crates/bytes-pool)
[![Documentation](https://docs.rs/bytes-pool/badge.svg)](https://docs.rs/bytes-pool)
![MIT licensed](https://img.shields.io/crates/l/bytes-pool.svg)
<br />
[![Dependency Status](https://deps.rs/crate/bytes-pool/latest/status.svg)](https://deps.rs/crate/bytes-pool)
![Downloads](https://img.shields.io/crates/d/bytes-pool.svg)

## Usage

To use `bytes-pool`, first add this to your `Cargo.toml`:

```toml
[dependencies]
bytes-pool = "1"
```

Next, add this to your crate:

```rust
use bytes_pool::BytesPool;
```

## no_std support

To use `bytes-pool` with no_std environment, disable the (enabled by default) `std` feature.

```toml
[dependencies]
bytes-pool = { version = "1", default-features = false }
```

`bytes-pool` forwards the `std` feature to `bytes`. It also forwards the `extra-platforms` feature if enabled. See the [no_std documentation for the `bytes` crate](https://docs.rs/crate/bytes/latest) for more information.

## Serde support

Serde support is optional and disabled by default. To enable use the feature `serde`.

```toml
[dependencies]
bytes-pool = { version = "1", features = ["serde"] }
```

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `bytes-pools` by you, shall be licensed as MIT, without any
additional terms or conditions.
