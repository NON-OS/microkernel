// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/process/context/cpu_context.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/process/context/cpu_context.rs"]
pub mod cpu_context;

pub fn cpucontext_new() -> cpu_context::CpuContext {
    cpu_context::CpuContext::new()
}

