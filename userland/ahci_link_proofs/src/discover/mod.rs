// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../capsule_driver_ahci/src/discover/found.rs"]
mod found;
/// The kernel's Intel VMD list, which the driver's rule must agree with.
#[cfg(test)]
#[path = "../../../../src/hardware/inventory/vmd.rs"]
pub mod kernel_vmd;
#[path = "../../../capsule_driver_ahci/src/discover/rule.rs"]
pub mod rule;

pub use found::Found;
