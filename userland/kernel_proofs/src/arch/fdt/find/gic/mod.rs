// NONOS Operating System (AGPL-3.0-or-later)
//! The GIC compatible list the I2C finder names its interrupt parent by.

#[path = "../../../../../../../src/arch/fdt/find/gic/compatible.rs"]
mod compatible;
#[allow(dead_code)]
#[path = "../../../../../../../src/arch/fdt/find/gic/version.rs"]
mod version;

pub use compatible::classify;
