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

//! The block wire statuses a SCSI command ends in when it fails.

pub const E_IO: i32 = -5;
pub const E_NXIO: i32 = -6;
pub const E_ACCES: i32 = -13;
pub const E_INVAL: i32 = -22;
pub const E_ROFS: i32 = -30;
pub const E_TIMEDOUT: i32 = -110;
/// A removable medium is not in the drive.
pub const E_NOMEDIUM: i32 = -123;
