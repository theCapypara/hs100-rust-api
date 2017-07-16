use std::fmt;
use std::error::Error as StdError;
use std::io::Error as IoError;
use serde_json::Error as JsonError;
use std::convert::From;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    EncryptError,
    DeserializeError(JsonError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(_) => f.write_str("Error connecting to the device"),
            Error::EncryptError => f.write_str("Failed to encrypt the message"),
            Error::DeserializeError(_) => {
                f.write_str("Couldn't deserialize the response received form the device")
            }
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(_) => "Error connecting to the device",
            Error::EncryptError => "Failed to encrypt message",
            Error::DeserializeError(_) => "Couldn't parse the response received form the device",
        }
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::IoError(error)
    }
}

impl From<JsonError> for Error {
    fn from(error: JsonError) -> Self {
        Error::DeserializeError(error)
    }
}