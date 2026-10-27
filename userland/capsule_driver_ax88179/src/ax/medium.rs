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

//! The PHY's status register (11h, the chip's own; as Linux v6.12 reads
//! it, read for behaviour) and the medium the MAC is set to for it: the
//! link (bit 10), full duplex (bit 13), and the speed in bits 15:14, 10b
//! gigabit, 01b 100 Mb/s, 00b 10 Mb/s. 11b is taken as 10 Mb/s, as Linux
//! takes it.

use super::regs::{M_100, M_125MHZ, M_FULL, M_GIGA, M_RECEIVE, M_RX_PAUSE, M_TX_PAUSE};

const LINK: u16 = 1 << 10;
const FULL: u16 = 1 << 13;
const SPEED: u16 = 0xC000;
const GIGABIT: u16 = 0x8000;
const FAST: u16 = 0x4000;

pub fn link_of(phy: u16) -> bool {
    phy & LINK != 0
}

/// The MEDIUM word for a PHY reporting `phy`: receive on, pause frames
/// both ways, and its speed and duplex.
pub fn medium_for(phy: u16) -> u16 {
    let speed = match phy & SPEED {
        GIGABIT => M_GIGA | M_125MHZ,
        FAST => M_100,
        _ => 0,
    };
    let duplex = if phy & FULL != 0 { M_FULL } else { 0 };
    M_RECEIVE | M_TX_PAUSE | M_RX_PAUSE | speed | duplex
}
