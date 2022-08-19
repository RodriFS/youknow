use std::ffi::OsString;
use std::str::Utf8Error;

use crate::file::File;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    OsStringError(OsString),
    Utf8Error(Utf8Error),
    ParseError,
    ReqwestError(reqwest::Error),
}

pub enum FileError<'a> {
    ReqwestError(&'a File, Error),
    IOError(&'a File, Error),
    ParseError(&'a File, Error),
    DeserializeError(&'a File, Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IOError(e)
    }
}

impl From<OsString> for Error {
    fn from(e: OsString) -> Error {
        Error::OsStringError(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Error {
        Error::Utf8Error(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::ReqwestError(e)
    }
}
