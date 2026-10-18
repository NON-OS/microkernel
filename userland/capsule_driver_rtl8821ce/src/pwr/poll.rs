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

//! The POLL step of a power sequence: read a byte register until its masked
//! value appears, with rtw88's PCIe retry.

use super::command::PwrCmd;
use crate::regs::Mmio;
use crate::wait::until_ms;

/// How long a poll step may take before it is declared failed: rtw88 polls
/// 20000 times, 50 us apart. Real transitions settle in microseconds; the
/// bound only stops a wedged card hanging the driver.
const POLL_MS: u64 = 1000;

const REG_SYS_PW_CTRL: usize = 0x0004;
const BIT_PFM_WOWL: u8 = 1 << 3;

/// rtw88 `rtw_pwr_cmd_polling` (mac.c:150): on PCIe a poll that times out is
/// retried once after pulsing BIT_PFM_WOWL in REG_SYS_PW_CTRL.
pub(super) fn poll<M: Mmio>(mmio: &M, step: &PwrCmd) -> bool {
    if poll_once(mmio, step) {
        return true;
    }
    let v = mmio.read8(REG_SYS_PW_CTRL);
    mmio.write8(REG_SYS_PW_CTRL, v | BIT_PFM_WOWL);
    mmio.write8(REG_SYS_PW_CTRL, v & !BIT_PFM_WOWL);
    poll_once(mmio, step)
}

fn poll_once<M: Mmio>(mmio: &M, step: &PwrCmd) -> bool {
    let want = step.value & step.mask;
    until_ms(POLL_MS, || mmio.read8(step.offset as usize) & step.mask == want)
}
