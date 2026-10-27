// NONOS Operating System (AGPL-3.0-or-later)

//! The PCI rings: channel registers, descriptor formats, the transmit and
//! receive rings, and their programming into the DMA engine.

pub mod addrs;
pub mod bd;
mod map;
pub mod rx;
pub mod set;
pub mod tx;
pub mod tx_rp;
pub mod wd;

pub use map::map_rings;
pub use set::Rings;
