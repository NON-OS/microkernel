// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/inventory/platform_sd.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/inventory/platform_sd.rs"]
pub mod platform_sd;

pub fn is_platform_sd_host(bus_kind: u8, class: u8, subclass: u8) -> bool {
    platform_sd::is_platform_sd_host(bus_kind, class, subclass)
}

