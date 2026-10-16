// NONOS Operating System (AGPL-3.0-or-later)
//! The PLIC compatible list the I2C finder names its interrupt parent by.

#[path = "../../../../../../../src/arch/fdt/find/plic/compatible.rs"]
mod compatible;

pub use compatible::matches;
