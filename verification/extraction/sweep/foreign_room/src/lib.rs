// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/process/foreign/room.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/process/foreign/room.rs"]
pub mod room;

pub fn has_room(held: usize) -> bool {
    room::has_room(held)
}

