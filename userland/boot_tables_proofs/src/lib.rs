// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 boot page tables, built by the kernel's own
//! code and walked back with the kernel's own walker.

pub mod arch;
pub mod sys;

pub use arch::aarch64::boot::info::{BootInfo, MemoryRegion, MemoryType, Window};
pub use arch::aarch64::mmu::probe;
