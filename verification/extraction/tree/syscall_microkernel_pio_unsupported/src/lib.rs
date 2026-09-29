// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/syscall/microkernel/pio/unsupported.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod syscall;

pub fn sys_pio_grant(_dev: u64, _epoch: u64, _bar: u8, _flags: u32, _out: u64) -> i64 {
    crate::syscall::microkernel::pio::unsupported::sys_pio_grant(_dev, _epoch, _bar, _flags, _out)
}

pub fn sys_pio_read(_grant: u64, _off: u64, _width: u64, _out: u64) -> i64 {
    crate::syscall::microkernel::pio::unsupported::sys_pio_read(_grant, _off, _width, _out)
}

pub fn sys_pio_write(_grant: u64, _off: u64, _width: u64, _value: u64) -> i64 {
    crate::syscall::microkernel::pio::unsupported::sys_pio_write(_grant, _off, _width, _value)
}

pub fn sys_pio_release(_grant: u64) -> i64 {
    crate::syscall::microkernel::pio::unsupported::sys_pio_release(_grant)
}

