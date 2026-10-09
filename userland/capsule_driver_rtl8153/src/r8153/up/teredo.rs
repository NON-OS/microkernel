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

//! The Teredo and wake offloads off (Linux r8153_teredo_off), so the
//! chip hands every frame to the host instead of answering some itself.

use nonos_usbnet::Bus;

use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{update_word, write_byte, write_dword, write_word, Dev, PLA};
use crate::r8153::regs::bits::{OOB_TEREDO_EN, TEREDO_RS_EVENT_MASK, TEREDO_SEL, WDT6_SET_MODE};
use crate::r8153::regs::pla::{REALWOW_TIMER, TEREDO_CFG, TEREDO_TIMER, WDT6_CTRL};
use crate::r8153::Version;

pub fn teredo_off<B: Bus>(dev: &mut Dev<B>, v: Version) -> Result<(), Fail> {
    let cfg = if v.is_8153b() {
        // On the RTL8153B bits 0 to 7 are write-one-to-clear.
        write_byte(dev, PLA, TEREDO_CFG, 0xff)
    } else {
        let off = TEREDO_SEL | TEREDO_RS_EVENT_MASK | OOB_TEREDO_EN;
        update_word(dev, PLA, TEREDO_CFG, off, 0)
    };
    at("Teredo offload not off", cfg)?;
    at("wake watchdog refused", write_word(dev, PLA, WDT6_CTRL, WDT6_SET_MODE))?;
    at("wake timer refused", write_word(dev, PLA, REALWOW_TIMER, 0))?;
    at("Teredo timer refused", write_dword(dev, PLA, TEREDO_TIMER, 0))
}
