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

//! Why an install stopped. Every variant names the fact, so the screen can
//! say which disk was too small, which sector the device refused, or where
//! the read-back first disagreed with what was sent.

use crate::fat32::WriteVolumeError;
use crate::sink::SinkError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteError {
    /// The device cannot hold the table plus the image.
    DiskTooSmall {
        total_sectors: u64,
        needed_sectors: u64,
    },
    Volume(WriteVolumeError),
    Sink(SinkError),
    /// The read-back differed from the source, first at this sector.
    Mismatch {
        lba: u64,
    },
}

impl From<WriteVolumeError> for WriteError {
    fn from(e: WriteVolumeError) -> Self {
        WriteError::Volume(e)
    }
}

impl From<SinkError> for WriteError {
    fn from(e: SinkError) -> Self {
        WriteError::Sink(e)
    }
}
