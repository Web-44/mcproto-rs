#![cfg_attr(feature = "bench", feature(test))]
#![cfg_attr(feature = "gat", feature(generic_associated_types))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(all(test, feature = "std", feature = "bench"))]
extern crate test;

pub mod byte_order;
mod chat;
mod deserialize;
pub mod nbt;
pub mod protocol;
mod serialize;
pub mod status;
pub mod types;
pub mod utils;
pub mod uuid;

#[cfg(feature = "v1_15_2")]
pub mod v1_15_2;
#[cfg(feature = "v1_16_3")]
pub mod v1_16_3;
#[cfg(feature = "v1_17_0")]
pub mod v1_17_0;

pub use deserialize::*;
pub use serialize::*;

#[cfg(all(test, feature = "std"))]
mod test_macros;
