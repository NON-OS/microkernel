// NONOS Operating System (AGPL-3.0-or-later)
//! Every architecture, held to the tree. `ledger` reads nonos-ci/arch-reach,
//! `scan` counts what the kernel's source names, `check` sets one against
//! the other. The std platform layer's one way into the kernel is held to
//! the registers the kernel reads (std_trap_tests).

pub mod check;
pub mod ledger;
pub mod rules;
pub mod scan;

#[cfg(test)]
mod capsule_tests;
#[cfg(test)]
mod check_tests;
#[cfg(test)]
mod form_tests;
#[cfg(test)]
mod ledger_tests;
#[cfg(test)]
mod scratch;
#[cfg(test)]
mod std_spawn_tests;
#[cfg(test)]
mod std_trap_tests;
#[cfg(test)]
mod tree_tests;
