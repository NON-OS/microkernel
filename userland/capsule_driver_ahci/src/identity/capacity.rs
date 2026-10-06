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

use super::address48::addresses_48_bit;
use super::refusal::Refusal;
use super::sector::logical_sector_bytes;
use crate::constants::ata::{LBA48_LIMIT, SECTOR_SIZE};
use crate::constants::identify::{IDENTIFY_WORDS, W_LBA48_CAPACITY};

/// The sector count the driver serves for a disk whose IDENTIFY block is
/// `words`. The rule: the 48-bit Address feature set is supported and
/// enabled, the logical sector is 512 bytes, and words 100-103 count at
/// least one sector and fewer than 2^48. That count is served; anything else
/// refuses the disk.
///
/// There is no fallback to the 28-bit count in words 60-61, which is never
/// read. Every command the driver issues (READ DMA EXT, WRITE DMA EXT,
/// FLUSH CACHE EXT) is a 48-bit one, so a disk without the feature set would
/// abort each of them: a 28-bit figure would serve a disk no request reaches.
///
/// The sector must be 512 bytes because that is the one size the wire format,
/// the PRD byte count and every copy are sized by, and the one the kernel
/// block layer addresses (it leaves NVMe and USB disks of other sizes unused
/// the same way). A 4096-byte disk is sane but not served here; a figure that
/// is no power of two, or outside 512..=4096, is refused all the more.
///
/// The bound is 2^48 because the H2D FIS carries 48 LBA bits. With a larger
/// count a request past 2^48 would pass the range check, the FIS would drop
/// its high bits, and the sectors near the start of the disk would be read or
/// written in its place.
pub fn capacity(words: &[u16; IDENTIFY_WORDS]) -> Result<u64, Refusal> {
    if !addresses_48_bit(words) {
        return Err(Refusal::No48BitAddress);
    }
    if logical_sector_bytes(words) != SECTOR_SIZE as u64 {
        return Err(Refusal::SectorSize);
    }
    let w = |i: usize| u64::from(words[i]);
    let sectors = w(W_LBA48_CAPACITY)
        | (w(W_LBA48_CAPACITY + 1) << 16)
        | (w(W_LBA48_CAPACITY + 2) << 32)
        | (w(W_LBA48_CAPACITY + 3) << 48);
    if sectors == 0 {
        return Err(Refusal::NoCapacity);
    }
    if sectors >= LBA48_LIMIT {
        return Err(Refusal::PastLba48);
    }
    Ok(sectors)
}
