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

//! The link word at PLA 0xe908, as the MAC sees the PHY: bit 1 link,
//! bit 0 full duplex, bits 2, 3 and 4 a speed of 10, 100 or 1000 Mb/s.

use super::reg::{pla16, Reg};
use super::regs::pla::LINK;

pub const WORD: Reg = pla16(LINK);
pub const FULL: u32 = 0x01;
pub const UP: u32 = 0x02;
pub const TEN: u32 = 0x04;
pub const HUNDRED: u32 = 0x08;

pub fn is_up(word: u32) -> bool {
    word & UP != 0
}

/// 10 or 100 Mb/s at half duplex, which takes the longer inter-frame gap.
pub fn half_slow(word: u32) -> bool {
    word & (TEN | HUNDRED) != 0 && word & FULL == 0
}
