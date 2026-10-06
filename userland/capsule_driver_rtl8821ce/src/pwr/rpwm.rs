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

//! The firmware wake rtw88 `rtw_mac_power_switch` sends first on a 3081-CPU
//! chip (mac.c:281): when firmware from an earlier boot is still running,
//! toggle the host power-mode bit so it lets go of the power state.

use crate::regs::Mmio;

/// `hci.rpwm_addr` for PCIe (main.c:1874).
const REG_PCIE_HRPWM: usize = 0x03D9;
/// `REG_MCUFW_CTRL`: reads 0xC078 while firmware is loaded and running.
const REG_MCUFW_CTRL: usize = 0x0080;
const FW_RUNNING: u16 = 0xC078;
/// `BIT_RPWM_TOGGLE`.
const RPWM_TOGGLE: u8 = 1 << 7;

/// Run before every power switch, as rtw88 does.
pub fn wake_firmware<M: Mmio>(mmio: &M) {
    let rpwm = mmio.read8(REG_PCIE_HRPWM);
    if mmio.read16(REG_MCUFW_CTRL) == FW_RUNNING {
        mmio.write8(REG_PCIE_HRPWM, (rpwm ^ RPWM_TOGGLE) & RPWM_TOGGLE);
    }
}
