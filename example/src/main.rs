extern crate hs100;

use hs100::SmartPlug;

const HOST: &'static str = "192.168.0.37:9999";

fn main() {
    let api = SmartPlug::new(HOST);

    let resp = api.sysinfo();
    println!("[sysinfo]: {}\n", resp);

    let resp = api.meterinfo();
    println!("[meterinfo]: {}", resp);
}
