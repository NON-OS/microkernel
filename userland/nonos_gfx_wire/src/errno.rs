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

//! The statuses a reply carries (section 12, "errors"); 0 is success.

pub type Errno = i32;

/// The output named is one the driver lacks.
pub const ENXIO: Errno = -6;
/// Not now; ask again (a flip still in flight).
pub const EAGAIN: Errno = -11;
/// No room: the display pool cannot hold the frame.
pub const ENOMEM: Errno = -12;
/// A primary still waits for RETIRE.
pub const EBUSY: Errno = -16;
/// A field out of range, a short body, a bad rectangle.
pub const EINVAL: Errno = -22;
/// The op is not served (a driver without HW_CURSOR, or before CAPS).
pub const ENOSYS: Errno = -38;
/// The device did not answer within the op's budget.
pub const ETIMEDOUT: Errno = -110;
/// A serial that is no longer the current primary.
pub const ESTALE: Errno = -116;
