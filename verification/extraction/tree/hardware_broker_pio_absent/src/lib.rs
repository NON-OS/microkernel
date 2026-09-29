// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/hardware/broker/pio_absent.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod hardware;

pub fn pio_release_all_for_pid(pid: u32) -> usize {
    crate::hardware::broker::pio_absent::pio_release_all_for_pid(pid)
}

pub fn pio_release_for_device(pid: u32, device_id: u64) -> usize {
    crate::hardware::broker::pio_absent::pio_release_for_device(pid, device_id)
}

