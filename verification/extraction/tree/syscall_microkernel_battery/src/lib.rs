// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/syscall/microkernel/battery.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;
pub mod syscall;

pub fn sys_battery_status() -> i64 {
    crate::syscall::microkernel::battery::sys_battery_status()
}

