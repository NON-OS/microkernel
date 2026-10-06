// NONOS Operating System (AGPL-3.0-or-later)
pub mod link;
#[path = "../../../capsule_driver_ahci/src/engine/completion.rs"]
pub mod completion;
#[path = "../../../capsule_driver_ahci/src/engine/prd_count.rs"]
pub mod prd_count;
#[path = "../../../capsule_driver_ahci/src/engine/span.rs"]
pub mod span;
// The recovery after a failed command, the engine steps and the kick it
// calls, run by the tests on a register file in host memory.
#[cfg(test)]
#[path = "../../../capsule_driver_ahci/src/engine/kick.rs"]
mod kick;
#[cfg(test)]
#[path = "../../../capsule_driver_ahci/src/engine/recover.rs"]
mod recover;
#[cfg(test)]
mod recover_tests;
#[cfg(test)]
#[path = "../../../capsule_driver_ahci/src/engine/start.rs"]
mod start;
#[cfg(test)]
#[path = "../../../capsule_driver_ahci/src/engine/stop.rs"]
mod stop;
