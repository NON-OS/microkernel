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

//! The card's size from its CSD, in 512-byte blocks, as mmc_decode_csd
//! (drivers/mmc/core/sd.c) reads each CSD structure version.

use super::bits::unstuff;

pub fn capacity_blocks(csd: &[u32; 4]) -> Option<u64> {
    match unstuff(csd, 126, 2) {
        // Version 1.0, standard capacity: (C_SIZE + 1) << (C_SIZE_MULT + 2)
        // blocks of READ_BL_LEN bytes.
        0 => {
            let c_size = unstuff(csd, 62, 12) as u64;
            let mult = unstuff(csd, 47, 3) as u64;
            let read_bl_len = unstuff(csd, 80, 4) as u64;
            let bytes = (c_size + 1) << (mult + 2 + read_bl_len);
            Some(bytes / 512)
        }
        // Version 2.0 (SDHC, SDXC): (C_SIZE + 1) * 512 KiB, C_SIZE 22 bits.
        1 => Some((unstuff(csd, 48, 22) as u64 + 1) << 10),
        // Version 3.0 (SDUC): the same with a 28-bit C_SIZE.
        2 => Some((unstuff(csd, 48, 28) as u64 + 1) << 10),
        _ => None,
    }
}

/// Whether a card of `blocks` is reached whole by a 32-bit argument: block
/// numbers on a high capacity card, byte offsets on a standard one. An
/// SDUC card past 2 TiB needs CMD22, which this driver does not send, and
/// a card of no blocks is no disk.
pub const fn addressable(blocks: u64, high_capacity: bool) -> bool {
    let limit = if high_capacity { 1 << 32 } else { 1 << 23 };
    blocks > 0 && blocks <= limit
}
