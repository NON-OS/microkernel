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

pub const E_OK: i32 = 0;
pub const E_INVAL: i32 = -22;
/// The sender may not reach the medium (`server::medium`).
pub const E_ACCES: i32 = -13;
pub const E_IO: i32 = -5;
pub const E_NXIO: i32 = -6;
pub const E_MSGSIZE: i32 = -90;
pub const E_NODEV: i32 = -19;
/// The command got no completion within its time.
pub const E_TIMEDOUT: i32 = -110;
/// Device statuses ride above the errnos: a command the disk ended with an
/// error answers -(ATA_STATUS_BASE | PxTFD.ERR << 8 | PxTFD.STS), so a failed
/// install can say what the disk said, not only that it failed. A client
/// that does not know the range reads it as an I/O error.
pub const ATA_STATUS_BASE: i32 = 0x1_0000;

/// The reply status for a command the disk failed with task file `tfd`.
pub const fn ata_status(tfd: u32) -> i32 {
    -(ATA_STATUS_BASE | (tfd & 0xffff) as i32)
}
