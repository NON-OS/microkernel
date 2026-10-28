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

//! The drawn station address written into the chip's address register,
//! which is locked against writes but for this window: the first four
//! bytes as a dword, the last two as a word. The chip's own address, in
//! its efuse, is never read. The receive filter (link_up.rs) takes
//! unicast to this address only.

use super::op::{put, Op};
use super::reg::{pla16, pla32, pla8};
use super::regs::pla::{CONFIG_LOCK, STATION};

const UNLOCKED: u32 = 0xc0;
const LOCKED: u32 = 0x00;

pub fn station(mac: [u8; 6]) -> [Op; 4] {
    let low = u32::from_le_bytes([mac[0], mac[1], mac[2], mac[3]]);
    let high = u16::from_le_bytes([mac[4], mac[5]]) as u32;
    [
        put(pla8(CONFIG_LOCK), UNLOCKED),
        put(pla32(STATION), low),
        put(pla16(STATION + 4), high),
        put(pla8(CONFIG_LOCK), LOCKED),
    ]
}
