// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/time/pit/channel.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn channel_data_port(this: crate::arch::x86_64::time::pit::channel::Channel) -> u16 {
    this.data_port()
}

pub fn channel_select_bits(this: crate::arch::x86_64::time::pit::channel::Channel) -> u8 {
    this.select_bits()
}

pub fn channel_readback_bit(this: crate::arch::x86_64::time::pit::channel::Channel) -> u8 {
    this.readback_bit()
}

