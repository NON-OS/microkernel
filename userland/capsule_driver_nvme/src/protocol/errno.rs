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

pub const E_INVAL: i32 = -22;
/// The sender may not reach the medium (`server::medium`).
pub const E_ACCES: i32 = -13;
pub const E_IO: i32 = -5;
pub const E_NXIO: i32 = -6;
pub const E_NODEV: i32 = -19;
pub const E_MSGSIZE: i32 = -90;
/// The command got no completion within its time.
pub const E_TIMEDOUT: i32 = -110;
/// Device statuses ride above the errnos: a command the controller completed
/// with an error answers -(DEVICE_STATUS_BASE | SCT << 8 | SC), so a failed
/// install can say what the controller said, not only that it failed. A
/// client that does not know the range reads it as an I/O error.
pub const DEVICE_STATUS_BASE: i32 = 0x1000;

/// The reply status for a completion with status code type `sct` and status
/// code `sc`.
pub const fn device_status(sct: u8, sc: u8) -> i32 {
    -(DEVICE_STATUS_BASE | ((sct as i32 & 0x7) << 8) | sc as i32)
}
