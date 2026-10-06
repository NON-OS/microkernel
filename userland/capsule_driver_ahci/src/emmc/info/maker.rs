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

//! The manufacturer name a JEDEC id stands for.

/// JEDEC manufacturer ids common on laptop eMMC, as Linux's mmc quirks
/// name them; anything else is called by the medium alone.
pub(super) fn maker(mid: u8) -> &'static [u8] {
    match mid {
        0x11 => b"Toshiba",
        0x13 | 0xfe => b"Micron",
        0x15 => b"Samsung",
        0x45 => b"SanDisk",
        0x70 => b"Kingston",
        0x90 => b"SK hynix",
        _ => b"eMMC",
    }
}
