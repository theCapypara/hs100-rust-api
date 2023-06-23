#[macro_use]
extern crate serde_derive;
#[cfg(feature = "async")]
use async_std::{net::TcpStream as AsyncTcpStream, prelude::*};
use byteorder::{BigEndian, WriteBytesExt};
use std::str;
use std::time::Duration;
#[cfg(feature = "sync")]
use std::{io::prelude::*, net::TcpStream};

pub mod error;
pub mod types;

use error::*;
use types::*;

pub struct SmartPlug {
    ip: &'static str,
}

impl SmartPlug {
    pub fn new(ip: &'static str) -> SmartPlug {
        SmartPlug { ip }
    }

    /// Wakes up the device
    #[maybe_async::maybe_async]
    pub async fn on(&self) -> Result<PlugInfo, Error> {
        let json = "{\"system\":{\"set_relay_state\":{\"state\":1}}}";
        self.submit_to_device(json).await
    }

    /// Turns off the device
    #[maybe_async::maybe_async]
    pub async fn off(&self) -> Result<PlugInfo, Error> {
        let json = "{\"system\":{\"set_relay_state\":{\"state\":0}}}";
        self.submit_to_device(json).await
    }

    /// Gather system wide info such as model of the device, etc.
    #[maybe_async::maybe_async]
    pub async fn sysinfo(&self) -> Result<PlugInfo, Error> {
        let json = "{\"system\":{\"get_sysinfo\":{}}}";
        self.submit_to_device(json).await
    }

    /// Gather system information as well as watt meter information
    #[maybe_async::maybe_async]
    pub async fn meterinfo(&self) -> Result<PlugInfo, Error> {
        let json = "{\"system\":{\"get_sysinfo\":{}}, \"emeter\":{\"get_realtime\":{},\"get_vgain_igain\":{}}}";
        self.submit_to_device(json).await
    }

    /// Returns system information as well as daily statistics of power usage
    #[maybe_async::maybe_async]
    pub async fn dailystats(&self, month: i32, year: i32) -> Result<PlugInfo, Error> {
        let json = format!(
            "{{\"emeter\":{{\"get_daystat\":{{\"month\":{},\"year\":{}}}}}}}",
            month, year
        );
        self.submit_to_device(&json).await
    }

    #[maybe_async::maybe_async]
    async fn submit_to_device(&self, msg: &str) -> Result<PlugInfo, Error> {
        let msg = encrypt(msg)?;
        let mut resp = send(self.ip, &msg).await?;
        let data = decrypt(&mut resp.split_off(4));

        // deserialize json
        let resp = serde_json::from_str(&data)?;

        Ok(resp)
    }
}

/// Prepare and encrypt message to send to the device
/// see: https://www.softscheck.com/en/reverse-engineering-tp-link-hs110/
fn encrypt(plain: &str) -> Result<Vec<u8>, Error> {
    let len = plain.len();
    let msgbytes = plain.as_bytes();
    let mut cipher = vec![];
    cipher.write_u32::<BigEndian>(len as u32)?;

    let mut key = 0xAB;
    let mut payload: Vec<u8> = Vec::with_capacity(len);

    for i in 0..len {
        payload.push(msgbytes[i] ^ key);
        key = payload[i];
    }

    for i in &payload {
        cipher.write_u8(*i)?;
    }

    Ok(cipher)
}

/// Decrypt received string
/// see: https://www.softscheck.com/en/reverse-engineering-tp-link-hs110/
#[allow(clippy::needless_range_loop)]
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

/// Sends a message to the device and awaits a response synchronously
#[maybe_async::sync_impl]
fn send(ip: &str, payload: &[u8]) -> Result<Vec<u8>, Error> {
    let mut stream = TcpStream::connect(ip)?;

    stream.set_read_timeout(Some(Duration::new(5, 0)))?;
    stream.write_all(payload)?;

    let mut resp = vec![];
    stream.read_to_end(&mut resp)?;

    Ok(resp)
}

/// Sends a message to the device and awaits a response asynchronously
#[maybe_async::async_impl]
async fn send(ip: &str, payload: &[u8]) -> Result<Vec<u8>, Error> {
    let mut stream = AsyncTcpStream::connect(ip).await?;

    let mut resp = vec![];
    async_std::io::timeout(Duration::new(5, 0), async {
        stream.write_all(payload).await?;
        stream.read_to_end(&mut resp).await?;
        Ok(())
    })
    .await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::decrypt;
    use super::encrypt;

    #[test]
    fn encrypt_decrypt() {
        let json = "{\"system\":{\"get_sysinfo\":{}}}";

        let mut data = encrypt(json).unwrap();
        let resp = decrypt(&mut data.split_off(4));

        assert_eq!(json, resp);
    }
}
