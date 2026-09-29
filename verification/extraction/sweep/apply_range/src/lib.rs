// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/elf/reloc/apply/range.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/elf/reloc/apply/range.rs"]
pub mod range;

pub fn in_range(addr: u64, size: u64, start: u64, seg_size: u64) -> bool {
    range::in_range(addr, size, start, seg_size)
}

