// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/boot/multicore/state.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/boot/multicore/state.rs"]
pub mod state;

pub fn online_cpu_count() -> u32 {
    state::online_cpu_count()
}

pub fn is_cpu_online(cpu: u32) -> bool {
    state::is_cpu_online(cpu)
}

pub fn wait_for_cpus(count: u32) {
    state::wait_for_cpus(count)
}

