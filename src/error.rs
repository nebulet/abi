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

macro_rules! error_impl {
    ($name:ident) => {
        pub const $name : Error = Error::new(concat_idents!(ERR_, $name));
    };
}

/// Error implementations
impl Error {
    error_impl!(INTERNAL);
    error_impl!(NOT_SUPPORTED);
    error_impl!(NO_RESOURCES);
    error_impl!(NO_MEMORY);
    error_impl!(INVALID_ARG);
    error_impl!(BAD_HANDLE);
    error_impl!(WRONG_TYPE);
    error_impl!(OUT_OF_BOUNDS);
    error_impl!(BUFFER_TOO_SMALL);
    error_impl!(BAD_STATE);
    error_impl!(TIMED_OUT);
    error_impl!(SHOULD_WAIT);
    error_impl!(CANCELLED);
    error_impl!(PEER_CLOSED);
    error_impl!(NOT_FOUND);
    error_impl!(ALREADY_EXISTS);
    error_impl!(ALREADY_OWNED);
    error_impl!(UNAVAILABLE);
    error_impl!(ACCESS_DENIED);
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