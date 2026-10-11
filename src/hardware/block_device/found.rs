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

//! What asking one disk came to.

use super::gpt::Places;
use super::BlockDeviceError;

pub(super) enum Found {
    /// The store header or the disk plan is where `places` puts it; `boot`
    /// when the disk's table holds the partition the loader came from.
    Layout { places: Places, boot: bool },
    /// No driver, no disk, or a disk without either structure.
    Absent,
    /// The backend could not be asked now; the answer may differ later.
    Refused(BlockDeviceError),
}

impl Found {
    /// A stopped driver or a disk that cannot serve the sector is not this
    /// disk; a refused caller or a lost reply says nothing about the disk.
    pub(super) fn of(e: BlockDeviceError) -> Found {
        match e {
            BlockDeviceError::Stale
            | BlockDeviceError::AccessDenied
            | BlockDeviceError::NoCallerPid
            | BlockDeviceError::TransportFailure
            | BlockDeviceError::ProtocolMismatch
            | BlockDeviceError::NotReady => Found::Refused(e),
            _ => Found::Absent,
        }
    }
}
