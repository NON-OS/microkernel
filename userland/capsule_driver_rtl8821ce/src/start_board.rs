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
    if !pcie::hold_link_awake(&mapped.regs) {
        status::line(b"[rtl8821ce] pcie link config did not answer\n");
    }
    // As rtw88 does for this chip: a late completion through VT-d must not
    // time out and stall the card's DMA.
    if pcie::disable_completion_timeout(mapped.device_id, mapped.claim_epoch) {
        status::line(b"[rtl8821ce] pcie completion timeout disabled\n");
    } else {
        status::line(b"[rtl8821ce] pcie completion timeout left on: write refused\n");
    }
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
