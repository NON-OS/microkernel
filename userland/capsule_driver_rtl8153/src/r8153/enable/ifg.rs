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

//! The inter-frame gap for the speed the link came up at (Linux
//! rtl_set_ifg): 144 ns on a half-duplex 10 or 100 Mb/s link, 96 ns
//! otherwise.

use nonos_usbnet::Bus;

use crate::r8153::ocp::{update_word, Dev, PLA};
use crate::r8153::regs::bits::{FULL_DUP, IFG_144NS, IFG_96NS, IFG_MASK, SPEED_10, SPEED_100};
use crate::r8153::regs::pla::TCR1;

/// `speed` is PLA_PHYSTATUS as rtl8152_get_speed reads it.
pub fn ifg<B: Bus>(dev: &mut Dev<B>, speed: u16) -> Result<(), i32> {
    let slow_half = speed & (SPEED_10 | SPEED_100) != 0 && speed & FULL_DUP == 0;
    let gap = if slow_half { IFG_144NS } else { IFG_96NS };
    update_word(dev, PLA, TCR1, IFG_MASK, gap)
}
