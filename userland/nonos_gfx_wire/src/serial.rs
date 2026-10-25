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

//! Primary serials are the driver's pid in the high half and a counter
//! in the low half (section 12), so a serial from a restarted driver
//! never matches one the compositor still holds.

pub fn serial(pid: u32, counter: u32) -> u32 {
    (pid << 16) | (counter & 0xFFFF)
}

/// The pid half of a serial.
pub fn pid_of(serial: u32) -> u32 {
    serial >> 16
}
