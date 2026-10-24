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
//! USB Attached SCSI (USB UAS 1.0 and T10 UAS, every layout as recalled,
//! unverified): the mass-storage interface's UAS alternate (protocol 62h)
//! and its four pipes, named by Pipe Usage descriptors; the information
//! units sent on the command pipe and read on the status pipe; tags; and
//! the steps of one command. The SCSI command is a `ScsiCommand`, which a
//! one-file adapter fits to whatever SCSI crate the caller has. Pure.

#![no_std]

mod command;
mod exchange;
mod heed;
mod iu;
mod pipe;
mod pipes;
mod status;
mod step;
mod tags;

pub use command::{DataDir, ScsiCommand};
pub use exchange::Exchange;
pub use heed::DROPS_MAX;
pub use iu::{command_iu, lun_bytes, task_iu, COMMAND_IU_LEN, TASK_IU_LEN};
pub use iu::{TMF_ABORT_TASK, TMF_LOGICAL_UNIT_RESET};
pub use pipe::{Pipe, UasPipes};
pub use pipes::uas_pipes;
pub use status::{status_iu, StatusIu};
pub use step::Step;
pub use tags::Tags;
