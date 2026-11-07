// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/process/exit/kept_table.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/process/exit/kept_table.rs"]
pub mod kept_table;

pub fn kept_new() -> kept_table::Kept {
    kept_table::Kept::new()
}

