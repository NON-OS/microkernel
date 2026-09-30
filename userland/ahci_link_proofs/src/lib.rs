// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the AHCI link-up predicates and the served-port choice. A
//! directory tree mirroring the driver's module path lets the included files'
//! `crate::constants::regs` and `super::candidate` paths resolve unchanged.

pub mod choose;
pub mod constants;
pub mod engine;

#[cfg(test)]
mod choose_tests;
#[cfg(test)]
mod sig_tests;
#[cfg(test)]
mod tests;
