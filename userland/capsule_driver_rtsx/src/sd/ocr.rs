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

//! The operating conditions exchange: CMD8's echo and ACMD41's OCR (SD
//! Physical Layer 4.3.13 and 5.1).

/// CMD8 argument: 2.7 to 3.6 V (VHS 0001b) and the check pattern AAh.
pub const IF_COND_ARG: u32 = 0x1AA;

const OCR_BUSY: u32 = 1 << 31;
const OCR_CCS: u32 = 1 << 30;

/// A card that speaks version 2.00 or later echoes both the voltage and
/// the pattern in R7.
pub const fn if_cond_echoed(r7: u32) -> bool {
    r7 & 0xFFF == IF_COND_ARG
}

/// Bit 31 is set once the card has finished powering up.
pub const fn ocr_ready(ocr: u32) -> bool {
    ocr & OCR_BUSY != 0
}

/// Card Capacity Status: SDHC, SDXC or SDUC, addressed by block.
pub const fn ocr_high_capacity(ocr: u32) -> bool {
    ocr & OCR_CCS != 0
}
