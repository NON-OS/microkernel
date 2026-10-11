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

//! The 128-bit register an R2 response carries, and its bit fields.

/// The 128-bit register an R2 response carries. The host stores response
/// bits 127:8 in its four response words' bits 119:0 (the CRC byte is
/// dropped), so the register is those 120 bits shifted up by eight, with
/// the CRC field reading zero (Linux sdhci_read_rsp_136).
pub const fn r2(words: [u32; 4]) -> u128 {
    let v = (words[0] as u128)
        | (words[1] as u128) << 32
        | (words[2] as u128) << 64
        | (words[3] as u128) << 96;
    v << 8
}

/// Bits hi..=lo of a 128-bit register.
pub const fn bits(reg: u128, hi: u32, lo: u32) -> u32 {
    let width = hi - lo + 1;
    ((reg >> lo) & ((1u128 << width) - 1)) as u32
}
