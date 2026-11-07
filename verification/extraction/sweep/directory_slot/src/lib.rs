// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/paging/manager/directory/slot.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/paging/manager/directory/slot.rs"]
pub mod slot;

pub fn slot_empty() -> slot::Slot {
    slot::Slot::empty()
}

