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

use crate::constants::{VIRTIO_BLK_S_IOERR, VIRTIO_BLK_S_OK, VIRTIO_BLK_S_UNSUPP};
use crate::queue::Fault;

/// How a request ended, as the handlers answer it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlkError {
    /// The device failed the request, or the queue would not take it.
    Io,
    /// The device refused the request's type (VIRTIO_BLK_S_UNSUPP).
    Refused,
    /// The device did not answer by the deadline: E_TIMEDOUT on the wire,
    /// told apart from a media error.
    Timeout,
}

impl From<Fault> for BlkError {
    fn from(fault: Fault) -> Self {
        match fault {
            Fault::TimedOut => BlkError::Timeout,
            _ => BlkError::Io,
        }
    }
}

/// The request's end, from the status byte the device wrote.
pub fn ended(status: u8) -> Result<(), BlkError> {
    match status {
        VIRTIO_BLK_S_OK => Ok(()),
        VIRTIO_BLK_S_IOERR => Err(BlkError::Io),
        VIRTIO_BLK_S_UNSUPP => Err(BlkError::Refused),
        _ => Err(BlkError::Io),
    }
}
