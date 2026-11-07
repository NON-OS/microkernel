// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/syscall/microkernel/acpi/owner_gate.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/syscall/microkernel/acpi/owner_gate.rs"]
pub mod owner_gate;

pub fn owner_only(nr: u64) -> bool {
    owner_gate::owner_only(nr)
}

