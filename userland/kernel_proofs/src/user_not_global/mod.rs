// NONOS Operating System (AGPL-3.0-or-later)
//! A user page never GLOBAL (P-06). With CR4.PGE on, a GLOBAL entry outlives
//! the CR3 write that leaves an address space, so a GLOBAL user entry would
//! stay readable to the next process on that CPU. Both kernel encoders drop
//! GLOBAL from a user mapping: `PagePermissions::to_pte_flags`, mounted with
//! the rest of `memory::paging`, and `PageTableEntry::to_raw`, mounted here.

#[path = "mmu/mod.rs"]
pub mod mmu;

#[cfg(test)]
mod tests;
