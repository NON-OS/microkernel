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

//! The SCSI command layer the host drivers share.
//!
//! A SCSI host adapter, real or virtual, moves a command descriptor block
//! to a target and a logical unit, moves the data the command names, and
//! hands back a status byte and, on CHECK CONDITION, sense data. Everything
//! above that is the same on every adapter: which CDB to build, what the
//! sense means and whether to try again, how to tell a disk from a CD drive
//! and how large it is. That is here, once, so virtio-scsi, Fusion-MPT,
//! PVSCSI, BusLogic and storvsc each only implement `Hba`.
//!
//! Pure and allocation free; every module is proved by its own tests.

#![cfg_attr(not(test), no_std)]

pub mod capacity;
pub mod cdb;
pub mod disk;
pub mod errno;
pub mod hba;
pub mod inquiry;
pub mod luns;
pub mod sense;
#[cfg(test)]
pub mod sim;
pub mod status;

pub use cdb::Cdb;
pub use disk::{command, flush, max_blocks, probe, read, refresh, write, Disk, Kind, Probe};
pub use hba::{Addr, Dir, Fault, Hba, Outcome, SENSE_MAX};
