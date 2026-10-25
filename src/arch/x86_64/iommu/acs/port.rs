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

//! One switch downstream port's ACS controls, read and closed at boot (L72).

use super::rule::{closed, closing, find_ext, ACS_ID};
use super::write::write_extended16;
use crate::drivers::pci::config::read_extended32;

/// Whether the port at `bus:device.function` is closed once this returns:
/// already so, or offering every needed control and set and read back.
pub(super) fn close(bus: u8, device: u8, function: u8) -> bool {
    let read = |at: u16| read_extended32(bus, device, function, at);
    let Some(at) = find_ext(read, ACS_ID) else { return false };
    let words = read(at + 4);
    if words == !0 {
        return false;
    }
    let (cap, ctl) = (words as u16, (words >> 16) as u16);
    if closed(ctl) {
        return true;
    }
    let Some(want) = closing(cap, ctl) else { return false };
    write_extended16(bus, device, function, at + 6, want) && closed((read(at + 4) >> 16) as u16)
}
