extern crate hs100api;

use hs100api::SmartPlug;
use hs100api::error::Error;

const HOST: &'static str = "192.168.0.37:9999";

fn main() {
    let api = SmartPlug::new(HOST);

    // Quick example:
    println!("[sysinfo]: {:?}\n", api.sysinfo());
    println!("[meterinfo]: {:?}\n", api.meterinfo());
    println!("[dailystats]: {:?}\n", api.dailystats(7, 2017));

    // Using it properly
    match api.sysinfo() {
        Ok(info) => println!("[sysinfo]: {:?}\n", info),
        Err(err) => println!("{}\n", err),
    }

    // Handle specific error types
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

    // Print specific property, it is "safe" to unwrap as long as:
    //    - for meterinfo() -> get_realtime
    //    - for dailystats() -> get_daystat
    //    - otherwise -> use match to handle if Option result is `Some` or `None`
    //
    //    note: this is just an example, this will panic on unwrap() if meterinfo()
    //       returns an error, use proper error handling as shown above.
    println!(
        "[watt]: {}",
        api.meterinfo()
            .unwrap()
            .emeter
            .unwrap()
            .get_realtime
            .unwrap()
            .current
    );

    //
    // Avoid these if the HS100 is plugged to your computer :)
    //

    // let resp = api.off();
    // let resp = api.on();
}
