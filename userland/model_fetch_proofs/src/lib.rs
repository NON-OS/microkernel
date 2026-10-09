// NONOS Operating System (AGPL-3.0-or-later)
/*
 * The model fetcher's catalogue reader and the pins it checks entries
 * against, compiled from the capsule's own sources; and the install of a
 * Qwen tier from the store, in every pure part it passes through.
 */

extern crate alloc;

#[path = "../../capsule_model_fetch/src/pins/mod.rs"]
pub mod pins;

pub mod catalogue;
/* The check of a tier's memory before its download, and the sizes it says. */
#[path = "../../capsule_model_fetch/src/get/memory_need.rs"]
pub mod memory_need;
#[path = "../../capsule_model_fetch/src/need.rs"]
pub mod need;
#[path = "../../capsule_model_fetch/src/size.rs"]
pub mod size;
/* Which files taking a tier away removes. */
#[path = "../../capsule_model_fetch/src/remove/plan.rs"]
pub mod remove_plan;
pub mod install;
pub mod net;
pub mod store;
/* The path a download takes, what is said of it, and where it stands. */
#[path = "../../capsule_model_fetch/src/path.rs"]
pub mod path;
#[path = "../../capsule_model_fetch/src/get/offer.rs"]
pub mod offer;
#[path = "../../capsule_model_fetch/src/get/eta.rs"]
pub mod eta;
#[path = "../../capsule_model_fetch/src/get/retry.rs"]
pub mod retry;
#[path = "../../capsule_model_fetch/src/get/anyone_wait.rs"]
pub mod anyone_wait;
#[path = "../../capsule_model_fetch/src/status_wire.rs"]
pub mod status_wire;
#[path = "../../capsule_model_fetch/src/status_write.rs"]
pub mod status_write;
#[path = "../../capsule_model_fetch/src/status_read.rs"]
pub mod status_read;
/* Init's reading of a direct download asked for with a tier's install. */
#[cfg(test)]
#[path = "../../../src/userspace/init/linux_jobs/direct.rs"]
mod init_direct;

// pins includes hex privately; the tests take their own copy of the decoder.
#[cfg(test)]
#[allow(clippy::duplicate_mod)]
#[path = "../../capsule_linux/src/linux/file/models/hex.rs"]
mod hex;
#[cfg(test)]
mod card_tests;
#[cfg(test)]
mod fit_tests;
#[cfg(test)]
mod install_tests;
#[cfg(test)]
mod memory_tests;
#[cfg(test)]
mod route_tests;
#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "../../capsule_model_fetch/src/errno.rs"]
mod errno;
#[cfg(test)]
mod errno_tests;
#[cfg(test)]
mod remove_tests;
#[cfg(test)]
mod path_tests;
#[cfg(test)]
mod retry_tests;
#[cfg(test)]
mod anyone_tests;
