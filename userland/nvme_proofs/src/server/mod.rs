// NONOS Operating System (AGPL-3.0-or-later)
#[cfg(test)]
mod bounds_tests;
pub mod handlers;
// The real order of the power-off and its reply.
#[path = "../../../capsule_driver_nvme/src/server/power_off_rule.rs"]
pub mod power_off_rule;
// The real request taking, which drops a recovery's bound once the inbox
// is seen empty.
#[path = "../../../capsule_driver_nvme/src/server/take.rs"]
pub mod take;
