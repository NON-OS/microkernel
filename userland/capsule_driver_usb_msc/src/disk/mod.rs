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

//! The bound device: BOT transport, recovery, and the SCSI commands the
//! kernel's block client needs.

mod bot;
mod data_phase;
mod lun;
mod ready;
mod recover;
mod scsi_io;
mod types;

pub use lun::max_lun;
pub use ready::{capacity, sync_cache, unit_ready};
pub use scsi_io::{read, write};
pub use types::Disk;
