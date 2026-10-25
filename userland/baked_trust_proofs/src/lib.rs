// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for which trust set a kernel build may embed. The build
//! script's module is the kernel's own file.

#![allow(dead_code)]

#[path = "../../../build_baked_trust.rs"]
pub mod build_baked_trust;

#[cfg(test)]
mod refusal_tests;
