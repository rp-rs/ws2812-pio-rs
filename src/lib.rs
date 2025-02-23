#![no_std]
#[cfg(feature = "rp2040")]
mod rp2040;

#[cfg(feature = "rp235x")]
mod rp235x;

#[cfg(feature = "rp2040")]
pub use rp2040::*;

#[cfg(feature = "rp235x")]
pub use rp235x::*;

#[cfg(all(feature = "rp2040", feature = "rp235x"))]
compile_error!("Only one HAL feature can be enabled at a time!");
