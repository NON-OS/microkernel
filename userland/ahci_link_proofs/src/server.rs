// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's pure server rules from its own files: who may ask for what,
//! and the power-off's order and time.

#[path = "../../capsule_driver_ahci/src/server/medium_rule.rs"]
pub mod medium_rule;
#[path = "../../capsule_driver_ahci/src/server/power_off_rule.rs"]
pub mod power_off_rule;
