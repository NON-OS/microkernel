// NONOS Operating System (AGPL-3.0-or-later)

//! The 8822C's efuse: a 512-byte physical map whose last 124 bytes are the
//! protected area the trims live in, unpacked into a 768-byte logical map.
//! The stall record (`diag`) is the RTL8821CE's.

mod deshuffle;
#[path = "../../../capsule_driver_rtl8821ce/src/efuse/diag.rs"]
mod diag;
mod dump;
pub mod facts;
mod info;
mod layout;

pub(crate) use deshuffle::deshuffle;
pub use diag::{diag, EfuseDiag};
pub(crate) use dump::dump_physical;
pub use info::{read, EfuseInfo};
