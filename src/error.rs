use core::{fmt, result};
use core::option::NoneError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Eq, PartialEq)]
pub struct Error {
    pub errno: i32,
}

impl Error {
    pub const fn new(errno: i32) -> Error {
        Error {
            errno: errno,
        }
    }

    pub fn mux(result: Result<u32>) -> u64 {
        match result {
            Ok(value) => value as u64,
            Err(error) => -error.errno as u64,
        }
    }

    pub fn demux(value: u64) -> Result<u32> {
        let errno = -(value as i32);
        if errno > 0 && errno < ERROR_STR.len() as i32 {
            Err(Error::new(errno))
        } else {
            // truncate
            Ok(value as u32)
        }
    }

    pub fn text(&self) -> &str {
        if let Some(desc) = ERROR_STR.get(-self.errno as usize) {
            desc
        } else {
            "An Unknown error occurred."
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
    fn from(_: NoneError) -> Error {
        Error::new(ERR_INTERNAL)
    }
}

/// Error implementations
impl Error {
    pub const INTERNAL: Error = Error::new(ERR_INTERNAL);
    pub const NOT_SUPPORTED: Error = Error::new(ERR_NOT_SUPPORTED);
    pub const NO_RESOURCES: Error = Error::new(ERR_NO_RESOURCES);
    pub const NO_MEMORY: Error = Error::new(ERR_NO_MEMORY);
    pub const INVALID_ARG: Error = Error::new(ERR_INVALID_ARG);
    pub const BAD_HANDLE: Error = Error::new(ERR_BAD_HANDLE);
    pub const WRONG_TYPE: Error = Error::new(ERR_WRONG_TYPE);
    pub const OUT_OF_BOUNDS: Error = Error::new(ERR_OUT_OF_BOUNDS);
    pub const BUFFER_TOO_SMALL: Error = Error::new(ERR_BUFFER_TOO_SMALL);
    pub const BAD_STATE: Error = Error::new(ERR_BAD_STATE);
    pub const TIMED_OUT: Error = Error::new(ERR_TIMED_OUT);
    pub const SHOULD_WAIT: Error = Error::new(ERR_SHOULD_WAIT);
    pub const CANCELLED: Error = Error::new(ERR_CANCELLED);
    pub const PEER_CLOSED: Error = Error::new(ERR_PEER_CLOSED);
    pub const NOT_FOUND: Error = Error::new(ERR_NOT_FOUND);
    pub const ALREADY_EXISTS: Error = Error::new(ERR_ALREADY_EXISTS);
    pub const ALREADY_OWNED: Error = Error::new(ERR_ALREADY_OWNED);
    pub const UNAVAILABLE: Error = Error::new(ERR_UNAVAILABLE);
    pub const ACCESS_DENIED: Error = Error::new(ERR_ACCESS_DENIED);
}

/// The system encountered an otherwise unspecified error
/// while performing the operation.
pub const ERR_INTERNAL: i32 = -1;

/// The operation is not implemented, supported, or enabled.
pub const ERR_NOT_SUPPORTED: i32 = -2;

/// The system was unable to allocate some resource
/// needed for the operation.
pub const ERR_NO_RESOURCES: i32 = -3;

/// The system was not able to allocate memory
/// needed for the operation.
pub const ERR_NO_MEMORY: i32 = -4;

/// An argument passed was invalid.
pub const ERR_INVALID_ARG: i32 = -5;

/// A specified handle value does not refer to a valid handle.
pub const ERR_BAD_HANDLE: i32 = -6;

/// The subject of the operation is the wrong type to
/// perform the operation.
pub const ERR_WRONG_TYPE: i32 = -7;

/// An argument is outside of the valid range for
/// this operation.
pub const ERR_OUT_OF_BOUNDS: i32 = -8;

/// A caller provided buffer is too small for
/// this operation.
pub const ERR_BUFFER_TOO_SMALL: i32 = -9;

/// The operation failed because the current state
/// of the object does not allow it, or a precondition
/// of the operation is not satisfied.
pub const ERR_BAD_STATE: i32 = -10;

/// The time limit for the operation elapsed before
/// the operation completed.
pub const ERR_TIMED_OUT: i32 = -11;

/// The operation cannot be performed currently but
/// potentially could succeed if the caller waits for a prerequisite
/// to be satisfied, for example waiting for a handle to be readable
/// or writable.
pub const ERR_SHOULD_WAIT: i32 = -12;

/// The in-progress operation (e.g. a wait) has been
/// canceled.
pub const ERR_CANCELLED: i32 = -13;

/// The operation failed because the remote end of the
/// subject of the operation was closed.
pub const ERR_PEER_CLOSED: i32 = -14;

/// The requested entity is not found.
pub const ERR_NOT_FOUND: i32 = -15;

/// An object with the specified identifier
/// already exists.
pub const ERR_ALREADY_EXISTS: i32 = -16;

/// The operation failed because the named entity
/// is already owned or controlled by another entity. The operation
/// could succeed later if the current owner releases the entity.
pub const ERR_ALREADY_OWNED: i32 = -17;

/// The subject of the operation is currently unable
/// to perform the operation.
pub const ERR_UNAVAILABLE: i32 = -18;

/// The caller did not have permission to perform
/// the specified operation.
pub const ERR_ACCESS_DENIED: i32 = -19;
/// ...
/// TODO: Add more errors

pub static ERROR_STR: [&'static str; 19] = [
    "The operation succeeded.",
    "The system encountered an otherwise unspecified error while performing the operation.",
    "The operation is not implemented, supported, or enabled.",
    "The system was not able to allocate some resource needed for the operation.",
    "The system was not able to allocate memory needed for the operation.",
    "An argument passed was invalid.",
    "A specified handle value does not refer to a valid handle.",
    "The subject of the operation is the wrong type to perform the operation.",
    "An argument is outside of the valid range for this operation.",
    "A caller provided buffer is too small for this operation.",
    "The operation failed because the current state of the object does not allow it, or a precondition of the operation is not satisfied.",
    "The time limit for the operation elapsed before the operation completed.",
    "The operation cannot be performed currently.",
    "The operation failed because the remote end of the subject of the operation was closed.",
    "The requested entity is not found.",
    "An object with the specified identifier already exists.",
    "The operation failed because the specified entity is already owned or controlled.",
    "The subject of the operation is currently unable to perform the operation.",
    "The caller did not have permission to perform the specified operation."
];
