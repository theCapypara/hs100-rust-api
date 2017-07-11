extern crate hs100;

use hs100::{encrypt, decrypt, send};

const HOST: &'static str = "192.168.0.37:9999";

fn main() {
    let json = "{\"system\":{\"get_sysinfo\":{}}}";

    // encrypt
    let msg = encrypt(json);
    let mut data = send(HOST, &msg);
    let resp = decrypt(&mut data.split_off(4));

    println!("resp: {}", resp)
}
