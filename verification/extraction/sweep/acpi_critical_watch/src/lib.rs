// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/acpi/critical_watch.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/acpi/critical_watch.rs"]
pub mod critical_watch;

pub fn watch_new() -> critical_watch::Watch {
    critical_watch::Watch::new()
}

