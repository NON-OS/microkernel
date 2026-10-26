// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 roster index. `roster_order` is the kernel's
//! own file.

#[path = "../../../src/arch/aarch64/boot/multicore/roster_order.rs"]
pub mod roster_order;

#[cfg(test)]
mod arrange_tests;
#[cfg(test)]
mod generated_tests;
#[cfg(test)]
mod hw_id_tests;
#[cfg(test)]
mod sweep_tests;
#[cfg(test)]
mod users_tests;
