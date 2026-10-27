// NONOS Operating System (AGPL-3.0-or-later)

//! The things the included RTL8852BE driver files take from `nonos_libc`.

// The real libc's register accessors, mounted so the driver's register
// window compiles against the code it ships with (the proofs never map one:
// they run the driver over the modelled chip instead).
#[path = "../../../libc/src/broker/dma_order.rs"]
mod dma_order;
#[path = "../../../libc/src/broker/mmio_access.rs"]
mod mmio_access;
pub use dma_order::{dma_mb, dma_rmb, dma_wmb};
pub use mmio_access::{
    mmio_read16, mmio_read32, mmio_read64, mmio_read8, mmio_write16, mmio_write32, mmio_write64,
    mmio_write8,
};

mod broker;
mod console;
mod services;
pub mod types;
pub use broker::*;
pub use console::*;
pub use services::*;
