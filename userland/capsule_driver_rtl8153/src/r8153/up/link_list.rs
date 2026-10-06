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

//! Out of the out-of-band (wake) mode, the MCU's hold on the buffers
//! given back, and the RX link list rebuilt: the middle of Linux
//! r8153_first_init, with wait_oob_link_list_ready after each change.

use nonos_usbnet::Bus;

use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{read_byte, update_byte, update_word, wait_until, Dev, PLA};
use crate::r8153::regs::bits::{LINK_LIST_READY, MCU_BORW_EN, NOW_IS_OOB, RE_INIT_LL};
use crate::r8153::regs::pla::{OOB_CTRL, SFF_STS_7};

/// Linux looks 1000 times, 1 to 2 ms apart.
const READY_MS: u64 = 2_000;

pub fn leave_oob<B: Bus>(dev: &mut Dev<B>) -> Result<(), Fail> {
    at("out-of-band mode not left", update_byte(dev, PLA, OOB_CTRL, NOW_IS_OOB, 0))?;
    at("MCU buffer hold not cleared", update_word(dev, PLA, SFF_STS_7, MCU_BORW_EN, 0))?;
    link_list_ready(dev)?;
    at("link list not rebuilt", update_word(dev, PLA, SFF_STS_7, 0, RE_INIT_LL))?;
    link_list_ready(dev)
}

fn link_list_ready<B: Bus>(dev: &mut Dev<B>) -> Result<(), Fail> {
    let ready = |d: &mut Dev<B>| Ok(read_byte(d, PLA, OOB_CTRL)? & LINK_LIST_READY != 0);
    at("link list not ready", wait_until(dev, READY_MS, 1, ready))
}
