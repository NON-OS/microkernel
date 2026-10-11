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

//! Both rings as the part walks them. A descriptor is the part's while OWN
//! is set; it reads the byte count as two's complement with ones on top and
//! refuses one without them, and it puts the frame check sequence on the end
//! of every frame it receives.

pub const OWN: u16 = 1 << 15;
pub const ERR: u16 = 1 << 14;
pub const CRC: u16 = 1 << 11;
pub const BUFF: u16 = 1 << 10;
pub const STP: u16 = 1 << 9;
pub const ENP: u16 = 1 << 8;
pub const TINT: u16 = 1 << 9;
pub const RINT: u16 = 1 << 10;
pub const MISS: u16 = 1 << 12;
pub const FCS: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];

#[derive(Debug, PartialEq, Eq)]
pub enum Delivered {
    Taken,
    NoBuffer,
    NotRunning,
}

pub fn buffer_len(bcnt: u16) -> usize {
    assert_eq!(bcnt & 0xF000, 0xF000, "byte count {bcnt:#x} without its ones");
    0x1000 - (bcnt & 0x0FFF) as usize
}
