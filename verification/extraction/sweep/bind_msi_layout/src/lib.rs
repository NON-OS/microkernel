// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/irq/bind/msi_layout.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/irq/bind/msi_layout.rs"]
pub mod msi_layout;

pub fn enable_one(ctrl: u16) -> u16 {
    msi_layout::enable_one(ctrl)
}

pub fn disable(ctrl: u16) -> u16 {
    msi_layout::disable(ctrl)
}

pub fn open_first(multi_message_capable: u8) -> u32 {
    msi_layout::open_first(multi_message_capable)
}

