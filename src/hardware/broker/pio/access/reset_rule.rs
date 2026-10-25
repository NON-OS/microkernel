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

//! The writes to a granted port that would reset the machine.
//!
//! The PS/2 driver holds 0x60-0x64, and the i8042's command port at 0x64
//! takes two commands that drive the controller's output lines, one of which
//! is wired to the CPU's reset: 0xF0-0xFF pulse them, and 0xD1 writes the
//! output port outright with the next byte. Either resets the processor with
//! no ZeroState wipe, from a capsule that only had to read keys. No driver
//! needs them, so the broker refuses them; every other command the driver
//! sends goes as before. A wider write is judged by the byte it puts at 0x64.

/// The i8042 command port.
pub(super) const I8042_COMMAND: u16 = 0x64;

/// Whether writing `value`, `width` bytes wide, at `port` would put a reset
/// command on the i8042.
pub(super) fn resets_machine(port: u16, width: u8, value: u32) -> bool {
    let Some(at) = I8042_COMMAND.checked_sub(port) else {
        return false;
    };
    if u32::from(at) >= u32::from(width) {
        return false;
    }
    let command = (value >> (8 * u32::from(at))) as u8;
    command == 0xD1 || command >= 0xF0
}
