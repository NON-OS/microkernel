// NONOS Operating System (AGPL-3.0-or-later)

//! The things the included driver files take from `nonos_libc`.

// The real libc's register accessors and DMA barriers, mounted so the driver
// files under test reach a device through the code they ship with.
#[path = "../../../libc/src/broker/dma_order.rs"]
mod dma_order;
#[path = "../../../libc/src/broker/mmio_access.rs"]
mod mmio_access;
pub use dma_order::{dma_mb, dma_rmb, dma_wmb};
pub use mmio_access::{
    mmio_read16, mmio_read32, mmio_read64, mmio_read8, mmio_write16, mmio_write32, mmio_write64,
    mmio_write8,
};

/// The driver's debug line (`status::debug`): written nowhere in the proofs.
pub fn mk_debug(_buf: *const u8, _len: usize) -> i64 {
    0
}

mod broker;
mod services;
mod time;
pub mod types;
pub use broker::*;
pub use services::*;
pub use time::*;
