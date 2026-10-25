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

//! The bytes keccak256 is taken over, besides the domain's 160, which are
//! nonos_nox_config's domain_words_at. Both typehashes are the config's
//! pinned ones, so this file holds no hash of its own.

use alloc::vec::Vec;

use super::entry::Entry;

/// typehash | the field words, which admit() has already checked. Every
/// field is a static type, so encodeData is the words as they came.
pub fn struct_preimage(e: &Entry, words: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(32 + words.len());
    out.extend_from_slice(&e.typed.typehash);
    out.extend_from_slice(words);
    out
}

/// 0x19 0x01 | domainSeparator | structHash.
pub fn digest_preimage(domain_separator: &[u8; 32], struct_hash: &[u8; 32]) -> [u8; 66] {
    let mut out = [0u8; 66];
    out[0] = 0x19;
    out[1] = 0x01;
    out[2..34].copy_from_slice(domain_separator);
    out[34..66].copy_from_slice(struct_hash);
    out
}
