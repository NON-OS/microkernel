// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the riscv64 kernel. `kernel` is the kernel's own file; it
//! assembles only for riscv64, where the tests run under qemu-riscv64.

#[cfg(target_arch = "riscv64")]
#[path = "../../../src/arch/riscv64/context/kernel.rs"]
pub mod kernel;

#[cfg(test)]
#[path = "../../../src/arch/riscv64/boot/memory_span.rs"]
pub mod memory_span;

#[cfg(test)]
mod boot_tests;
#[cfg(all(test, target_arch = "riscv64"))]
mod context_tests;
#[cfg(test)]
mod fence_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod source;
#[cfg(test)]
mod span_tests;
#[cfg(test)]
mod stop_tests;
