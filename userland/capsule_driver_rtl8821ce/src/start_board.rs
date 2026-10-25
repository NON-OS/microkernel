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

//! Start-up on a freshly powered MAC: the PCIe link held awake and the board
//! facts read.

use crate::setup::Mapped;
use crate::{efuse, pcie, status};

/// The PCIe link held awake and the board facts read, on a freshly powered
/// MAC.
pub fn link_and_board(mapped: &mut Mapped) {
    // Hold the link out of L1 before reading anything that matters. An L1 link
    // gates the chip's internal clocks, and a gated block answers reads with
    // zeros while the always-on registers keep answering, which reads as a
    // register window that is half alive. The host had ASPM L1 enabled on this
    // link, and the chip carries its own enable that only the driver can clear.
    match pcie::hold_link_awake(&mapped.regs) {
        Ok(()) => {}
        Err(pcie::DbiTimeout::Read) => {
            status::line(b"[rtl8821ce] pcie link config: dbi read timed out\n")
        }
        Err(pcie::DbiTimeout::Write) => {
            status::line(b"[rtl8821ce] pcie link config: dbi write timed out\n")
        }
    }
    // The PCIe completion timeout stays on, unlike rtw88, which turns it off for
    // this chip (plan/50 C1): a completion that never comes ends in an error
    // the bring-up names, never in a DMA engine waiting for it forever.
    // Read the board facts here, on a freshly powered MAC, which is where rtw88
    // takes them. Taken at the end of bring-up instead, after the firmware
    // download and the MAC tables, the efuse control and LDO registers answered
    // zero on real silicon while the chip id register a few offsets away answered
    // normally. A failure here is not fatal to the rest of bring-up: the PHY needs
    // the RF front-end option, so it reports the efuse stage itself.
    mapped.efuse = efuse::read(&mapped.regs);
    if mapped.efuse.is_none() {
        status::line(b"[rtl8821ce] efuse unreadable on a freshly powered mac\n");
    }
}
