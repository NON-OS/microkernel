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

//! The OCR bits the driver reads: power up done, access mode, voltages.

/// OCR bit 31: the card has finished power up (not busy).
pub const OCR_READY: u32 = 1 << 31;
/// OCR access mode, bits 30:29: 10b sector mode, 00b byte mode.
pub const OCR_ACCESS_MASK: u32 = 3 << 29;
pub const OCR_SECTOR_MODE: u32 = 2 << 29;
/// OCR voltage bits 23:7.
pub const OCR_VOLTAGE_MASK: u32 = 0x00ff_ff80;

pub const fn ocr_ready(ocr: u32) -> bool {
    ocr & OCR_READY != 0
}

pub const fn ocr_sector_mode(ocr: u32) -> bool {
    ocr & OCR_ACCESS_MASK == OCR_SECTOR_MODE
}
