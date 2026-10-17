// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 switch at an IRQ's return to EL0. Each reads
//! the kernel's own source and checks the order and settings it keeps.

#[cfg(test)]
mod el0_return_tests;
#[cfg(test)]
mod softfloat_tests;
#[cfg(test)]
mod source;
#[cfg(test)]
mod switch_in_tests;
#[cfg(test)]
mod switch_out_tests;
