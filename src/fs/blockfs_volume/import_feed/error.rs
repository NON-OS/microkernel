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

/* Why a streamed import refused a step. */

use super::super::error::VolumeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamError {
    /* The volume, or the disk under it, refused. */
    Volume(VolumeError),
    /* Another running process holds the stream. */
    Busy,
    /* This process has no stream begun. */
    NotBegun,
    /* More bytes than the length named at the start. */
    Overrun,
    /* Finished before every byte arrived; the stream is kept. */
    Short,
    /* The volume has no room for the bytes still to come. */
    NoRoom,
}

impl From<VolumeError> for StreamError {
    fn from(e: VolumeError) -> Self {
        StreamError::Volume(e)
    }
}
