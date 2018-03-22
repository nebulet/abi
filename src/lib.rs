#![no_std]
#![feature(try_trait)]
#![feature(concat_idents)]
#![feature(const_fn)]

pub use self::call::*;
pub use self::error::*;

/// ABI calls
pub mod call;

/// All the errors that can be generated by abi calls
pub mod error;