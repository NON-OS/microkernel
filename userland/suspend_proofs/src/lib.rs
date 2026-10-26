// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the powerdown idle path. `layout` is the kernel's own
//! file; `header` and `asm` read `asm/suspend.h` and `asm/suspend.S`.

#[path = "../../../src/arch/aarch64/idle_states/layout.rs"]
pub mod layout;

#[cfg(test)]
mod asm;
#[cfg(test)]
mod header;
#[cfg(test)]
mod keys_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod model;
#[cfg(test)]
mod model_tests;
#[cfg(test)]
mod ops;
#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod resume_map_tests;
