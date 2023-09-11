# BrownAppleJuice

Rust write of https://github.com/ECTO-1A/AppleJuice/ , mostly because managing python dependencies can be a pain at times.

Thanks to [ECTO-1A](https://github.com/ECTO-1A) for the initial python script, and [bluez/bluer](https://github.com/bluez/bluer/blob/cfc9363160f1a66f312592ae96b5a746350e8f02/bluer/examples/le_advertise.rs) for the BLE advertisement example.

## Building

Only Linux supported. You must be using the bluez Bluetooth stack, and have [libdbus](https://packages.debian.org/buster/libdbus-1-dev) (TBD). Make sure you've rust, e.g. via [rustup](https://rustup.rs/). To build, run:

```
cargo build --release
```

## Running

Run it via:

```
RUST_LOG=trace ./target/release/brown_apple_juice
```

You can omit the rust log env if you'd like.

