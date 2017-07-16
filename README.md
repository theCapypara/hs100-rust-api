HS100 API for Rust
====

[![CratesIo](https://img.shields.io/crates/v/hs100api.svg)](https://crates.io/crates/hs100api)

Simple library in Rust to access HS100/110 functions.

Special thanks to: https://github.com/sausheong/hs1xxplug for the Go version (which this library is basically a port from).

Resource on Reverse Engineering the HS110: https://www.softscheck.com/en/reverse-engineering-tp-link-hs110/

# Usage

```rust
extern crate hs100;

use hs100::SmartPlug;
use hs100::error::Error;

const HOST: &'static str = "192.168.0.37:9999"; // your device IP

fn main() {
    let api = SmartPlug::new(HOST);

    // Quick example:
    println!("[sysinfo]: {:?}\n", api.sysinfo());
    println!("[meterinfo]: {:?}\n", api.meterinfo());
    println!("[dailystats]: {:?}\n", api.dailystats(7, 2017));

    // Handle specific error types:
    match api.sysinfo() {
        Ok(info) => println!("[sysinfo]: {:?}\n", info),
        Err(err) => {
            match err {
                Error::IoError(_) => println!("some io error occurred"),
                Error::EncryptError => println!("error encrypting the message"),
                Error::DeserializeError(_) => println!("couldn't deserialize the message"),
            }
        }
    }

    // See the example folder for more usage patterns.
}
```

# TODO

- ~~Deserialize json into structs~~
- ~~Proper error handling~~
- Use Futures / asynchronous I/O

# License

MIT