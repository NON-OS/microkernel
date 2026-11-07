// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/confine/iova_space.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/confine/iova_space.rs"]
pub mod iova_space;

pub fn touches_interrupt_window(start: u64, length: u64) -> bool {
    iova_space::touches_interrupt_window(start, length)
}

pub fn after_give_back(next: u64, iova: u64, length: u64) -> u64 {
    iova_space::after_give_back(next, iova, length)
}

