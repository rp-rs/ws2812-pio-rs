#![no_std]

#![allow(unused_imports)]
#[cfg(feature = "rp2040")]
mod rp2040;

#[cfg(feature = "rp235x")]
mod rp235x;

#[cfg(feature = "rp2040")]
pub use rp2040::*;

#[cfg(feature = "rp235x")]
pub use rp235x::*;

