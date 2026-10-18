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

//! The statuses a driver answers with itself. A failed device command
//! answers what the driver makes of it (sdhci's `emmc::wire_status`): -110
//! for a timeout, a status carrying the device's own detail, or -5.

pub const E_NXIO: i32 = -6;
/// The sender may not reach the medium (`medium_rule`).
pub const E_ACCES: i32 = -13;
/// The part has no name to give (IDENTIFY).
pub const E_NODEV: i32 = -19;
pub const E_INVAL: i32 = -22;
pub const E_MSGSIZE: i32 = -90;
/// The card's write-protect switch is on.
pub const E_ROFS: i32 = -30;
