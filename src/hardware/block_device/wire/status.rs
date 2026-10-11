// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! A reply's status word as the client's error.

use super::error::WireError;

const E_NXIO: i32 = -6;
const E_AGAIN: i32 = -11;
const E_ACCES: i32 = -13;
const E_NODEV: i32 = -19;
const E_INVAL: i32 = -22;
const E_ROFS: i32 = -30;
const E_MSGSIZE: i32 = -90;
const E_TIMEDOUT: i32 = -110;
const E_NOMEDIUM: i32 = -123;

/// EIO, and anything the wire does not name, is a failed I/O, never success.
pub(super) fn lift(status: i32) -> WireError {
    match status {
        E_NXIO => WireError::OutOfRange,
        E_AGAIN => WireError::NotReady,
        E_ACCES => WireError::AccessDenied,
        E_NODEV => WireError::NoDevice,
        E_INVAL => WireError::InvalidArgument,
        E_ROFS => WireError::ReadOnly,
        E_MSGSIZE => WireError::OversizedRequest,
        E_TIMEDOUT => WireError::TimedOut,
        E_NOMEDIUM => WireError::NoMedium,
        _ => WireError::DeviceFailure,
    }
}
