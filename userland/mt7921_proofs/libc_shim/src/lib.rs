// NONOS Operating System (AGPL-3.0-or-later)

//! The calls the mounted MT7921 driver files make of `nonos_libc`, on the
//! host: the broker's DMA as host memory, the firmware package as slots a
//! proof fills, the clock as a counter, the console as a list of lines.

mod broker;
mod dma;
mod services;
mod source;
mod time;
pub mod types;

pub use broker::*;
pub use dma::*;
pub use services::*;
pub use source::*;
pub use time::*;

/// BAR0 is never mapped on the host: the proofs put the modelled chip
/// behind the driver's `Bus`, so a raw register read is all ones.
///
/// # Safety
/// Never dereferences `addr`.
pub unsafe fn mmio_read32(_addr: usize) -> u32 {
    u32::MAX
}

/// # Safety
/// Never dereferences `addr`.
pub unsafe fn mmio_write32(_addr: usize, _value: u32) {}
