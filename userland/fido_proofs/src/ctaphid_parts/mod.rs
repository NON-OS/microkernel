// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's report assembly, mounted on its own: the files that frame,
//! judge and put together CTAPHID reports, which refer to one another only
//! through `super`, so the sweep reaches them without the driver's exports.

#![allow(dead_code)]

#[path = "../../../capsule_driver_fido/src/ctaphid/assemble.rs"]
pub mod assemble;
#[path = "../../../capsule_driver_fido/src/ctaphid/assembled.rs"]
mod assembled;
#[path = "../../../capsule_driver_fido/src/ctaphid/frame.rs"]
pub mod frame;
#[path = "../../../capsule_driver_fido/src/ctaphid/heard.rs"]
pub mod heard;
#[path = "../../../capsule_driver_fido/src/ctaphid/opening.rs"]
mod opening;
