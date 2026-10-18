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

//! The PIIX bus master IDE function (82371AB data sheet, 2.7; SFF-8038i):
//! eight ports per channel, the primary at 0, the secondary at 8.

pub const fn channel_base(channel: usize) -> u16 {
    (channel * 8) as u16
}

/// Command: start, and the direction (set: the engine writes memory, a
/// read from the disk).
pub const BMIC: u16 = 0;
pub const BMIC_START: u8 = 1 << 0;
pub const BMIC_TO_MEMORY: u8 = 1 << 3;

/// Status: active while the engine moves data, error on a bus fault,
/// interrupt latched from INTRQ (write one to clear both), and the
/// "drive can DMA" bits firmware sets for each drive it found.
pub const BMIS: u16 = 2;
pub const BMIS_ACTIVE: u8 = 1 << 0;
pub const BMIS_ERROR: u8 = 1 << 1;
pub const BMIS_INTERRUPT: u8 = 1 << 2;
pub const fn bmis_drive_dma(drive: u8) -> u8 {
    1 << (5 + (drive & 1))
}

/// The descriptor table's bus address: four-byte aligned, 32 bits.
pub const BMIDTP: u16 = 4;

/// One descriptor: a 32-bit base, a 16-bit count (zero is 64 KiB) and end of
/// table in bit 15 of the last word. No region may cross a 64 KiB boundary.
pub const PRD_BYTES: usize = 8;
pub const PRD_EOT: u16 = 1 << 15;
pub const PRD_BOUNDARY: u64 = 1 << 16;
