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

use crate::constants::identify::{
    IDENTIFY_WORDS, SECTOR_LONGER_THAN_256_WORDS, WORD_VALID, WORD_VALID_MASK,
    W_LOGICAL_SECTOR_WORDS, W_SECTOR_SIZE,
};

/// The logical sector size, in bytes, that `words` gives. Word 106 speaks
/// only when its bits 15:14 read 01b; then bit 12 says the logical sector is
/// longer than 256 words and words 117-118 give its length in words.
/// Otherwise the sector is 256 words, 512 bytes. This is the decode Linux
/// makes (ata_id_logical_sector_size). The length is doubled in u64, so no
/// figure the drive writes can wrap to a small one.
///
/// The result is only ever compared with SECTOR_SIZE, never used to size a
/// buffer or a copy.
pub fn logical_sector_bytes(words: &[u16; IDENTIFY_WORDS]) -> u64 {
    let w106 = words[W_SECTOR_SIZE];
    if w106 & WORD_VALID_MASK != WORD_VALID || w106 & SECTOR_LONGER_THAN_256_WORDS == 0 {
        return 512;
    }
    let length_words = u64::from(words[W_LOGICAL_SECTOR_WORDS])
        | (u64::from(words[W_LOGICAL_SECTOR_WORDS + 1]) << 16);
    length_words * 2
}
