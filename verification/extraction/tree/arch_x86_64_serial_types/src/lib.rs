// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/serial/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn baudrate_divisor(this: crate::arch::x86_64::serial::types::BaudRate) -> u16 {
    this.divisor()
}

pub fn baudrate_from_divisor(divisor: u16) -> u32 {
    crate::arch::x86_64::serial::types::BaudRate::from_divisor(divisor)
}

pub fn baudrate_as_u32(this: crate::arch::x86_64::serial::types::BaudRate) -> u32 {
    this.as_u32()
}

pub fn databits_as_u8(this: crate::arch::x86_64::serial::types::DataBits) -> u8 {
    this.as_u8()
}

pub fn databits_bits(this: crate::arch::x86_64::serial::types::DataBits) -> u8 {
    this.bits()
}

pub fn parity_as_u8(this: crate::arch::x86_64::serial::types::Parity) -> u8 {
    this.as_u8()
}

pub fn stopbits_as_u8(this: crate::arch::x86_64::serial::types::StopBits) -> u8 {
    this.as_u8()
}

pub fn serialconfig_with_fifo(this: crate::arch::x86_64::serial::types::SerialConfig, enable: bool) -> crate::arch::x86_64::serial::types::SerialConfig {
    this.with_fifo(enable)
}

pub fn serialconfig_with_interrupts(this: crate::arch::x86_64::serial::types::SerialConfig, enable: bool) -> crate::arch::x86_64::serial::types::SerialConfig {
    this.with_interrupts(enable)
}

