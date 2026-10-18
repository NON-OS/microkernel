// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 ASID pool, mounted from the kernel.

extern crate alloc;

#[cfg(test)]
#[path = "../../../src/arch/aarch64/mmu/asid/pool.rs"]
mod pool;

#[cfg(test)]
mod pool_tests;
