// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/windows.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/windows.rs"]
pub mod windows;

pub fn touches_device_window(addr: u64, len: u64) -> bool {
    windows::touches_device_window(addr, len)
}

