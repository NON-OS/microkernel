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

/*
 * The keyring's OP_SIGN_TYPED_DATA (op 30, plan/31), as the wallet asks it
 * for entries 2 (PublisherBinding) and 3 (ReleaseAttest): pid u32 | key u32
 * | entry u16 | reserved u16 = 0 | chain_id u64 | n u8 | n words, LE. The
 * keyring's own table holds each entry's type string and domain, so the
 * wallet names neither. Op 30 is C's; it reaches this branch at the merge.
 */

use alloc::vec::Vec;

pub const OP_SIGN_TYPED_DATA: u16 = 30;
pub const ENTRY_BINDING: u16 = 2;
pub const ENTRY_ATTEST: u16 = 3;

pub fn typed_request(pid: u32, key: u32, entry: u16, chain_id: u64, words: &[[u8; 32]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(21 + 32 * words.len());
    out.extend_from_slice(&pid.to_le_bytes());
    out.extend_from_slice(&key.to_le_bytes());
    out.extend_from_slice(&entry.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&chain_id.to_le_bytes());
    out.push(words.len() as u8);
    words.iter().for_each(|w| out.extend_from_slice(w));
    out
}
