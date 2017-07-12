extern crate byteorder;

use std::io::prelude::*;
use std::net::TcpStream;
use byteorder::{BigEndian, WriteBytesExt};
use std::time::Duration;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::str;

mod types;
use types::*;

pub struct SmartPlug {
    ip: &'static str,
}

impl SmartPlug {
    pub fn new(ip: &'static str) -> SmartPlug {
        SmartPlug { ip: ip }
    }

    fn submit(&self, msg: &str) -> String {
        let msg = encrypt(msg);
        let mut data = self.send(self.ip, &msg);
        decrypt(&mut data.split_off(4))
    }

    pub fn on(&self) -> String {
        let json = "{\"system\":{\"set_relay_state\":{\"state\":1}}}";
        self.submit(json)
    }

    pub fn off(&self) -> String {
        let json = "{\"system\":{\"set_relay_state\":{\"state\":0}}}";
        self.submit(json)
    }

    pub fn sysinfo(&self) -> PlugInfo {
        let json = "{\"system\":{\"get_sysinfo\":{}}}";
        let data = self.submit(json);
        serde_json::from_str(&data).unwrap()
    }

    pub fn meterinfo(&self) -> PlugInfo {
        let json = "{\"system\":{\"get_sysinfo\":{}}, \"emeter\":{\"get_realtime\":{},\"get_vgain_igain\":{}}}";
        let data = self.submit(json);
        serde_json::from_str(&data).unwrap()
    }

    pub fn dailystats(&self, month: i32, year: i32) -> PlugInfo {
        let json = format!(
            "{{\"emeter\":{{\"get_daystat\":{{\"month\":{},\"year\":{}}}}}}}",
            month,
            year
        );
        let data = self.submit(&json);
        serde_json::from_str(&data).unwrap()
    }

    fn send(&self, ip: &str, payload: &[u8]) -> Vec<u8> {
        let mut stream = TcpStream::connect(ip).expect("Couldn't connect to the server...");
        stream.set_read_timeout(Some(Duration::new(5, 0))).expect(
            "set_read_timeout call failed",
        );
        stream.write_all(payload).unwrap();

        let mut response = vec![];
        stream.read_to_end(&mut response).expect("Could not read");

        response
    }
}

fn encrypt(plain: &str) -> Vec<u8> {
    let len = plain.len();
    let msgbytes = plain.as_bytes();
    let mut cipher = vec![];
    cipher.write_u32::<BigEndian>(len as u32).expect(
        "Can't write header",
    );

    let mut key = 0xAB;
    let mut payload: Vec<u8> = Vec::with_capacity(len);

    for i in 0..len {
        payload.push(msgbytes[i] ^ key);
        key = payload[i];
    }

    for i in &payload {
        cipher.write_u8(*i).expect("Can't write message");
    }

    cipher
}

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
