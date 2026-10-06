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

//! Why an install stopped, in the words both installers print. A refusal
//! for size names the size the disk has and the size NONOS needs.

use core::fmt;

use super::error::WriteError;
use crate::describe::size_text;
use crate::sink::SECTOR_SIZE;

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = |sectors: u64| size_text(sectors.saturating_mul(SECTOR_SIZE as u64));
        match *self {
            WriteError::DiskTooSmall { total_sectors, needed_sectors } => write!(
                f,
                "the disk holds {}; NONOS needs a disk of at least {} ({} MiB)",
                bytes(total_sectors),
                bytes(needed_sectors),
                needed_sectors.saturating_mul(SECTOR_SIZE as u64) >> 20
            ),
            WriteError::Volume(v) => write!(f, "the boot partition could not be laid out ({v:?})"),
            WriteError::Sink(s) => write!(f, "the disk refused a transfer (status {})", s.0),
            WriteError::Mismatch { lba } => {
                write!(f, "sector {lba} read back different from what was written")
            }
        }
    }
}
