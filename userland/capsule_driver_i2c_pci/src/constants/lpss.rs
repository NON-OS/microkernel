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
// Intel LPSS wrapper (Skylake and later, drivers/mfd/intel-lpss.c). The
// DesignWare core sits at 0x000..0x100; the LPSS private register block is at
// offset 0x200 inside the same MMIO BAR. Out of reset the LPSS holds the
// DesignWare core asserted, so the RESETS register must be deasserted before
// any core register is touched or every access returns garbage.
pub const LPSS_PRIV: u64 = 0x200;
pub const LPSS_PRIV_RESETS: u64 = LPSS_PRIV + 0x04;
// Bits 1:0 release the function (Linux LPSS_PRIV_RESETS_FUNC, coreboot
// LPSS_CNT_RST_RELEASE, Intel's RESETS_FUNC and RESETS_APB); bit 2 releases
// the integrated DMA engine (LPSS_PRIV_RESETS_IDMA). Writing 0 asserts all
// three. This driver moves every byte by programmed I/O, so the DMA engine,
// which could otherwise master the bus from the same window, stays held.
pub const LPSS_PRIV_RESETS_FUNC: u32 = 0x3;
// Intel LPSS private registers beyond the reset (intel_lpss_init_dev and
// intel_lpss_set_remap_addr): the 64-bit remap address the integrated DMA
// uses for the function's MMIO window, written low then high, and the
// capabilities word whose type field tells I2C (0) from UART (1) and SPI (2).
pub const LPSS_PRIV_REMAP_LO: u64 = LPSS_PRIV + 0x40;
pub const LPSS_PRIV_REMAP_HI: u64 = LPSS_PRIV + 0x44;
pub const LPSS_PRIV_CAPS: u64 = LPSS_PRIV + 0xFC;
pub const LPSS_PRIV_CAPS_TYPE_SHIFT: u32 = 4;
pub const LPSS_PRIV_CAPS_TYPE_MASK: u32 = 0xF;
pub const LPSS_DEV_I2C: u32 = 0;
