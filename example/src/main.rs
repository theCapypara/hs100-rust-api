extern crate hs100;

use hs100::SmartPlug;

const HOST: &'static str = "192.168.0.37:9999";

fn main() {
    let api = SmartPlug::new(HOST);

    println!("[sysinfo]: {:?}\n", api.sysinfo());
    println!("[meterinfo]: {:?}\n", api.meterinfo());
    println!("[dailystats]: {:?}\n", api.dailystats(7, 2017));

    // Print specific property, it's "safe" to unwrap as long as:
    //    for meterinfo() -> get_realtime
    //    for dailystats() -> get_daystat
    //    etc. see types.rs for more info on types using Options
    //
    //    otherwise -> use match and handle error case properly.
    println!(
        "[watt]: {}",
        api.meterinfo()
            .emeter
            .unwrap()
            .get_realtime
            .unwrap()
            .current
    )

    //
    // Avoid these if the HS100 is plugged to your computer :)
    //

    // let resp = api.off();
    // let resp = api.on();
}
