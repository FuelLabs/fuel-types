//! Atomic types of the FuelVM.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod types;

use bounded_integer::bounded_integer;
pub use types::*;

/// Word-aligned bytes serialization functions.
pub mod bytes;

/// Register ID type
pub type RegisterId = usize;

/// Register value type
pub type Word = u64;

bounded_integer! {
    /// 6-bits immediate value type
    pub struct Immediate06 { 0..64 }
}

bounded_integer! {
    /// 12-bits immediate value type
    pub struct Immediate12 { 0..4096 }
}

bounded_integer! {
    /// 18-bits immediate value type
    pub struct Immediate18 { 0..262144 }
}

bounded_integer! {
    /// 24-bits immediate value type
    pub struct Immediate24 { 0..16777216 }
}
