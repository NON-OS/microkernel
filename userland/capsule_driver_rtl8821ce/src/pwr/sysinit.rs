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

//! rtw88 `__rtw_mac_init_system_cfg` (mac.c:330), the 3081-CPU version the
//! 8821C takes, run straight after power-on (mac.c:401). Values from reg.h
//! and rtw8821c.c.

use crate::regs::Mmio;

const REG_GPIO_MUXCFG: usize = 0x0040;
const REG_SYS_FUNC_EN: usize = 0x0002;
const REG_MCUFW_CTRL: usize = 0x0080;
const REG_CPU_DMEM_CON: usize = 0x1080;
const REG_CR_EXT: usize = 0x1100;
/// BIT_WL_PLATFORM_RST | BIT_DDMA_EN.
const PLATFORM_RST_DDMA: u32 = (1 << 16) | (1 << 8);
/// The 8821C's `sys_func_en` (rtw8821c.c:2010).
const SYS_FUNC_EN_HI: u8 = 0xD8;
const BIT_BOOT_FSPI_EN: u32 = 1 << 20;
const BIT_FSPI_EN: u32 = 1 << 19;

/// Release the WLAN platform, enable DDMA and the system functions, and stop
/// the chip booting firmware from flash so the driver's download is used.
pub fn init_system_cfg<M: Mmio>(mmio: &M) {
    let v = mmio.read32(REG_CPU_DMEM_CON);
    mmio.write32(REG_CPU_DMEM_CON, v | PLATFORM_RST_DDMA);
    let f = mmio.read8(REG_SYS_FUNC_EN + 1);
    mmio.write8(REG_SYS_FUNC_EN + 1, f | SYS_FUNC_EN_HI);
    let c = mmio.read8(REG_CR_EXT + 3);
    mmio.write8(REG_CR_EXT + 3, (c & 0xF0) | 0x0C);
    let fw = mmio.read32(REG_MCUFW_CTRL);
    if fw & BIT_BOOT_FSPI_EN != 0 {
        mmio.write32(REG_MCUFW_CTRL, fw & !BIT_BOOT_FSPI_EN);
        let g = mmio.read32(REG_GPIO_MUXCFG);
        mmio.write32(REG_GPIO_MUXCFG, g & !BIT_FSPI_EN);
    }
}
