// NONOS Operating System (AGPL-3.0-or-later)
#[cfg(test)]
mod bounds_tests;
pub mod handlers;
// The real request taking, which drops a recovery's bound once the inbox
// is seen empty.
#[path = "../../../capsule_driver_nvme/src/server/take.rs"]
pub mod take;
