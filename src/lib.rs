extern crate byteorder;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::io::prelude::*;
use std::net::TcpStream;
use byteorder::{BigEndian, WriteBytesExt};
use std::time::Duration;
use std::str;

pub mod types;
pub mod error;

use types::*;
use error::*;

pub struct SmartPlug {
    ip: &'static str,
}

impl SmartPlug {
    pub fn new(ip: &'static str) -> SmartPlug {
        SmartPlug { ip: ip }
    }

    // Wakes up the device
    pub fn on(&self) -> Result<PlugInfo, Error> {
        let json = "{\"system\":{\"set_relay_state\":{\"state\":1}}}";
        self.submit_to_device(json)
    }

    // Turns off the device
    pub fn off(&self) -> Result<PlugInfo, Error> {
        let json = "{\"system\":{\"set_relay_state\":{\"state\":0}}}";
        self.submit_to_device(json)
    }

    // Gather system wide info such as model of the device, etc.
    pub fn sysinfo(&self) -> Result<PlugInfo, Error> {
        let json = "{\"system\":{\"get_sysinfo\":{}}}";
        self.submit_to_device(json)
    }

    // Gather system information as well as watt meter information
    pub fn meterinfo(&self) -> Result<PlugInfo, Error> {
        let json = "{\"system\":{\"get_sysinfo\":{}}, \"emeter\":{\"get_realtime\":{},\"get_vgain_igain\":{}}}";
        self.submit_to_device(json)
    }

    // Returns system information as well as daily statistics of power usage
    pub fn dailystats(&self, month: i32, year: i32) -> Result<PlugInfo, Error> {
        let json = format!(
            "{{\"emeter\":{{\"get_daystat\":{{\"month\":{},\"year\":{}}}}}}}",
            month,
            year
        );
        self.submit_to_device(&json)
    }

    fn submit_to_device(&self, msg: &str) -> Result<PlugInfo, Error> {
        let msg = try!(encrypt(msg));
        let mut resp = try!(send(self.ip, &msg));
        let data = decrypt(&mut resp.split_off(4));

        // deserialize json
        let resp = try!(serde_json::from_str(&data));

        Ok(resp)
    }
}

// Prepare and encrypt message to send to the device
// see: https://www.softscheck.com/en/reverse-engineering-tp-link-hs110/
fn encrypt(plain: &str) -> Result<Vec<u8>, Error> {
    let len = plain.len();
    let msgbytes = plain.as_bytes();
    let mut cipher = vec![];
    try!(cipher.write_u32::<BigEndian>(len as u32));

    let mut key = 0xAB;
    let mut payload: Vec<u8> = Vec::with_capacity(len);

    for i in 0..len {
        payload.push(msgbytes[i] ^ key);
        key = payload[i];
    }

    for i in &payload {
        try!(cipher.write_u8(*i));
    }

    Ok(cipher)
}

// Decrypt received string
// see: https://www.softscheck.com/en/reverse-engineering-tp-link-hs110/
fn decrypt(cipher: &mut [u8]) -> String {
    let len = cipher.len();

    let mut key = 0xAB;
    let mut next: u8;

    for i in 0..len {
        next = cipher[i];
        cipher[i] ^= key;
        key = next;
    }

    String::from_utf8_lossy(cipher).into_owned()
}

// Sends a message to the device and awaits a response synchronously
fn send(ip: &str, payload: &[u8]) -> Result<Vec<u8>, Error> {
    let mut stream = try!(TcpStream::connect(ip));

    try!(stream.set_read_timeout(Some(Duration::new(5, 0))));
    try!(stream.write_all(payload));

    let mut resp = vec![];
    try!(stream.read_to_end(&mut resp));

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use encrypt;
    use decrypt;

    #[test]
    fn encrypt_decrypt() {
        let json = "{\"system\":{\"get_sysinfo\":{}}}";

        let mut data = encrypt(json);
        let resp = decrypt(&mut data.split_off(4));

        assert_eq!(json, resp);
    }
}
