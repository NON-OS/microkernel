// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/dma/acpi_stop.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/dma/acpi_stop.rs"]
pub mod acpi_stop;

pub fn may_map(bus_kind: u8, class: u32) -> bool {
    acpi_stop::may_map(bus_kind, class)
}

