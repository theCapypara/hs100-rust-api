extern crate bincode;
extern crate byteorder;

use std::io::prelude::*;
use std::net::TcpStream;
use byteorder::{BigEndian, WriteBytesExt};
use std::time::Duration;
use std::str;

pub fn encrypt(plain: &str) -> Vec<u8> {
    let len = plain.len();
    let msgbytes = plain.as_bytes();
    let mut cipher = vec![];
    cipher.write_u32::<BigEndian>(len as u32).unwrap();

    let mut key = 0xAB;
    let mut payload: Vec<u8> = Vec::with_capacity(len);

    for i in 0..len {
        payload.push(msgbytes[i] ^ key);
        key = payload[i];
    }

    for i in 0..payload.len() {
        cipher.write_u8(payload[i]).unwrap();
    }

    cipher
}

pub fn decrypt(cipher: &mut [u8]) -> String {
    let len = cipher.len();

    let mut key = 0xAB;
    let mut next: u8;

    for i in 0..len {
        next = cipher[i];
        cipher[i] = cipher[i] ^ key;
        key = next;
    }

    String::from_utf8_lossy(cipher).into_owned()
}

pub fn send(ip: &str, payload: &[u8]) -> Vec<u8> {
    let mut stream = TcpStream::connect(ip).expect("Couldn't connect to the server...");
    stream.set_read_timeout(Some(Duration::new(5, 0))).expect(
        "set_read_timeout call failed",
    );
    stream.write_all(payload).unwrap();

    let mut response = vec![];
    stream.read_to_end(&mut response).expect("Could not read");

    response
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
