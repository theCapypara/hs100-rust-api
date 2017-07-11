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
