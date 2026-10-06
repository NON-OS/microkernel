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
    FEATURE_LBA48, IDENTIFY_WORDS, WORD_VALID, WORD_VALID_MASK, W_ENABLED_2, W_ENABLED_DEFAULT,
    W_SUPPORTED_2,
};

/// The disk supports the 48-bit Address feature set (word 83 bit 10) and has
/// it enabled (word 86 bit 10). Word 83 counts only when its own bits 15:14
/// read 01b, word 86 only when word 87's do: the same test Linux applies
/// (ata_id_has_lba48, ata_id_lba48_enabled). Without the validity bits an
/// all-ones block, which sets bit 10 everywhere, would pass.
pub fn addresses_48_bit(words: &[u16; IDENTIFY_WORDS]) -> bool {
    let valid = |w: u16| w & WORD_VALID_MASK == WORD_VALID;
    valid(words[W_SUPPORTED_2])
        && words[W_SUPPORTED_2] & FEATURE_LBA48 != 0
        && valid(words[W_ENABLED_DEFAULT])
        && words[W_ENABLED_2] & FEATURE_LBA48 != 0
}
