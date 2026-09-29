// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/process/scheduler/task/types/affinity.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod process;

pub fn cpuaffinity_any() -> crate::process::scheduler::task::types::affinity::CpuAffinity {
    crate::process::scheduler::task::types::affinity::CpuAffinity::any()
}

