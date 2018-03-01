use core::{fmt, result};
use core::option::NoneError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Eq, PartialEq)]
pub struct Error {
    pub errno: i32,
}

impl Error {
    pub fn new(errno: i32) -> Error {
        Error {
            errno: errno,
        }
    }

    pub fn mux(result: Result<usize>) -> usize {
        match result {
            Ok(value) => value,
            Err(error) => -error.errno as usize,
        }
    }

    pub fn demux(value: usize) -> Result<usize> {
        let errno = -(value as i32);
        if errno > 0 && errno < ERROR_STR.len() as i32 {
            Err(Error::new(errno))
        } else {
            Ok(value)
        }
    }

    pub fn text(&self) -> &str {
        if let Some(desc) = ERROR_STR.get(self.errno as usize) {
            desc
        } else {
            "Unknown error"
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.text())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.text())
    }
}

impl From<NoneError> for Error {
    fn from(error: NoneError) -> Error {
        Error::new(-1)
    }
}

pub const EPERM: i32 = 1; // Operation not permitted
/// ...
/// TODO: Add more errors

pub static ERROR_STR: [&'static str; 2] = [
    "Success",
    "Operation not permitted",
];