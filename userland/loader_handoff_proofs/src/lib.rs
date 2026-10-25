// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 loader handoff. `place` is the kernel's own
//! file.

#![allow(dead_code)]

#[path = "../../../src/arch/aarch64/boot/loader/place.rs"]
pub mod place;

#[cfg(test)]
mod install_tests;
#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod place_tests;
#[cfg(test)]
mod source;
