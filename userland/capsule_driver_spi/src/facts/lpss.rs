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
//! The LPSS wrapper around the SPI core, as for the I2C functions (the PCH
//! datasheets' LPSS chapter, as recalled, unverified): a private register
//! block at 0x200 of the same BAR holding the resets, the remap address and
//! the capabilities, and for SPI a chip-select control register. The
//! chip-select offset is the weakest fact here (recalled as private + 0x24
//! on Sunrise Point and later); the driver reads it back after each write
//! and refuses a function where the bit does not stick.

pub const PRIV: u64 = 0x200;
pub const PRIV_RESETS: u64 = PRIV + 0x04;
/// Function and both DMA resets released.
pub const PRIV_RESETS_DEASSERT: u32 = 0x7;
/// The capabilities register: its type field (bits 7:4) says I2C (0), UART
/// (1) or SPI (2).
pub const PRIV_CAPS: u64 = PRIV + 0xFC;
pub const CAPS_TYPE_SHIFT: u32 = 4;
pub const CAPS_TYPE_MASK: u32 = 0xF;
pub const CAPS_TYPE_SPI: u32 = 2;
/// Chip-select control: bit 0 software mode, bit 1 the chip select's level
/// in software mode (1 deasserted, as the line is active low).
pub const PRIV_CS_CONTROL: u64 = PRIV + 0x24;
pub const CS_SW_MODE: u32 = 1 << 0;
pub const CS_HIGH: u32 = 1 << 1;
/// The SPI core's input clock is not read from anywhere: the fastest an
/// LPSS generation uses is assumed (as recalled, unverified), so a divider
/// computed for it can only make the bus slower than asked, never faster.
pub const INPUT_CLOCK_HZ: u32 = 133_000_000;
