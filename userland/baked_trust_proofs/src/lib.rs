// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for which trust set a kernel build may embed. The build
//! script's rule is the kernel build's own file.

#[cfg(test)]
#[path = "../../../build_baked_trust_rule.rs"]
mod rule;

#[cfg(test)]
mod refusal_tests;
