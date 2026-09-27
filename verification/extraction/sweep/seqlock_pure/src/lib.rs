// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/sys/sync/seqlock/pure.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/sys/sync/seqlock/pure.rs"]
pub mod pure;

pub fn is_stable(seq: u32) -> bool {
    pure::is_stable(seq)
}

pub fn bump(seq: u32) -> u32 {
    pure::bump(seq)
}

pub fn read_valid(before: u32, after: u32) -> bool {
    pure::read_valid(before, after)
}

