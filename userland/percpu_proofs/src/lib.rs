// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the per-CPU block. `percpu` mirrors the kernel's module
//! so the `super::` paths inside the mounted files resolve; `operations`
//! is a stand-in holding a few blocks, as the kernel's static array does.

// The kernel's block has a const `new` and no `Default`; the proofs give it
// one (percpu/default.rs).
pub mod percpu;

#[cfg(test)]
mod asm_tests;
#[cfg(test)]
mod base_tests;
#[cfg(test)]
mod layout_tests;
