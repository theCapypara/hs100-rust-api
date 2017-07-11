HS100 API for Rust
====

Simple library in Rust to access HS100/110 functions.

Special thanks to: https://github.com/sausheong/hs1xxplug for the Go version (which this library is basically a port from).

Resource on Reverse Engineering the HS110: https://www.softscheck.com/en/reverse-engineering-tp-link-hs110/

# Usage

```rust
extern crate hs100;

use hs100::SmartPlug;

const HOST: &'static str = "192.168.0.37:9999";

fn main() {
    let api = SmartPlug::new(HOST);

    let resp = api.sysinfo();
    println!("[sysinfo]: {}\n", resp);

    let resp = api.meterinfo();
    println!("[meterinfo]: {}\n", resp);

    let resp = api.dailystats(7, 2017);
    println!("[dailystats]: {}", resp);

    //
    // Avoid these if the HS100 is plugged to your computer :)
    //

    // let resp = api.off();
    // let resp = api.on();
}
```

# TODO

- Deserialize json response to struct.

# License

MIT