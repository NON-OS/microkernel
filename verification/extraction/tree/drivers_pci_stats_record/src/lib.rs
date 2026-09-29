// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/drivers/pci/stats/record.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod drivers;

pub fn record_enumeration(time_us: u64) {
    crate::drivers::pci::stats::record::record_enumeration(time_us)
}

pub fn record_config_read() {
    crate::drivers::pci::stats::record::record_config_read()
}

pub fn record_config_write() {
    crate::drivers::pci::stats::record::record_config_write()
}

pub fn record_config_error() {
    crate::drivers::pci::stats::record::record_config_error()
}

pub fn record_interrupt(is_msi: bool) {
    crate::drivers::pci::stats::record::record_interrupt(is_msi)
}

pub fn record_hotplug_event() {
    crate::drivers::pci::stats::record::record_hotplug_event()
}

pub fn record_power_state_change() {
    crate::drivers::pci::stats::record::record_power_state_change()
}

pub fn record_link_state_change() {
    crate::drivers::pci::stats::record::record_link_state_change()
}

pub fn record_error_event() {
    crate::drivers::pci::stats::record::record_error_event()
}

pub fn reset_stats() {
    crate::drivers::pci::stats::record::reset_stats()
}

