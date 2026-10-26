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

//! The bound device: BOT or UAS transport, recovery, and the SCSI commands
//! the kernel's block client needs.

pub(crate) mod bot;
mod capacity;
mod csw;
mod data;
mod data_phase;
mod digest;
mod inquiry;
mod lun;
mod ready;
mod recover;
mod scsi_io;
mod sense;
mod serial;
mod transport;
pub(crate) mod types;
mod uas;
mod uas_cmd;
mod uas_data;

pub use capacity::{capacity, sync_cache};
pub use inquiry::inquiry;
pub use lun::max_lun;
pub use ready::{test_ready, Unit};
pub use scsi_io::{read, write};
pub use serial::serial;
pub use types::Disk;
