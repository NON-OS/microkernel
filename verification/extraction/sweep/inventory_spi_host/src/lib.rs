// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/inventory/spi_host.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/inventory/spi_host.rs"]
pub mod spi_host;

pub fn spi_host(bus: u8, vendor: u16, device: u16) -> bool {
    spi_host::spi_host(bus, vendor, device)
}

