// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/riscv64/context/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/riscv64/context/types.rs"]
pub mod types;

pub fn userentry_zeroed() -> types::UserEntry {
    types::UserEntry::zeroed()
}

pub fn saveduser_zeroed() -> types::SavedUser {
    types::SavedUser::zeroed()
}

