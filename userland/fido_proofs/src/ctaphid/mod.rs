// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The driver's CTAPHID core, each file mounted once at the path the driver
//! names (`crate::ctaphid`) with the driver's own exports; the files the
//! frame sweep drives are open to it as well.

#[path = "../../../capsule_driver_fido/src/ctaphid/assemble.rs"]
pub(crate) mod assemble;
#[path = "../../../capsule_driver_fido/src/ctaphid/assembled.rs"]
mod assembled;
#[path = "../../../capsule_driver_fido/src/ctaphid/frame.rs"]
pub(crate) mod frame;
#[path = "../../../capsule_driver_fido/src/ctaphid/heard.rs"]
pub(crate) mod heard;
#[path = "../../../capsule_driver_fido/src/ctaphid/init.rs"]
mod init;
#[path = "../../../capsule_driver_fido/src/ctaphid/job.rs"]
mod job;
#[path = "../../../capsule_driver_fido/src/ctaphid/job_end.rs"]
mod job_end;
#[path = "../../../capsule_driver_fido/src/ctaphid/key.rs"]
mod key;
#[path = "../../../capsule_driver_fido/src/ctaphid/opening.rs"]
mod opening;
#[path = "../../../capsule_driver_fido/src/ctaphid/talk.rs"]
mod talk;

pub use frame::{CMD_CANCEL, CMD_CBOR, PACKET};
pub use init::{Channel, CAP_CBOR};
pub use job::{Fault, Job};
pub use key::{find_keys, free_keys, Key};
pub use talk::{allocate, send};
