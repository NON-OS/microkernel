// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the EPD1 trampoline. `arch` and `spectre` mirror the
//! kernel's modules so the `super::` paths inside the mounted files resolve.

#![cfg(test)]

pub mod arch;
pub mod spectre;
#[path = "../../../src/arch/aarch64/security/epd1/step.rs"]
pub mod step;

#[cfg(test)]
mod answer_tests;
#[cfg(test)]
mod asm;
#[cfg(test)]
mod asm_tests;
#[cfg(test)]
mod classify_tests;
#[cfg(test)]
mod footprint_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod orders;
#[cfg(test)]
mod regions;
#[cfg(test)]
mod source;
#[cfg(test)]
mod step_tests;
#[cfg(test)]
mod vector_tests;
#[cfg(test)]
mod walk;
