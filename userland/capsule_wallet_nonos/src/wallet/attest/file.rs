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
 * The ReleaseAttest log on disk, /data/wallet.attests: fixed length, so a
 * short or long file is plainly not this one.
 *
 *   "NXAT" | version u16 = 1 | n u16 | MAX_ATTESTS (1024) slots of RECORD_LEN
 *   record = chain_id u64 | listing_id | release_id | bond_id | chunk_root
 *            | content_len u64 | package_hash | sig[65]
 *
 * The first n slots are used and every byte of the rest is zero.
 */

use alloc::vec::Vec;

use super::types::Record;

pub const MAX_ATTESTS: usize = 1024;
pub const RECORD_LEN: usize = 8 + 32 * 5 + 8 + 65;
pub const FILE_LEN: usize = 8 + MAX_ATTESTS * RECORD_LEN;
pub(super) const MAGIC: &[u8; 4] = b"NXAT";

pub fn encode(log: &[Record]) -> Option<Vec<u8>> {
    if log.len() > MAX_ATTESTS {
        return None;
    }
    let mut out = Vec::with_capacity(FILE_LEN);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&(log.len() as u16).to_le_bytes());
    for r in log {
        let a = &r.attest;
        out.extend_from_slice(&a.chain_id.to_le_bytes());
        for w in [&a.listing_id, &a.release_id, &a.bond_id, &a.chunk_root] {
            out.extend_from_slice(w);
        }
        out.extend_from_slice(&a.content_len.to_le_bytes());
        out.extend_from_slice(&a.package_hash);
        out.extend_from_slice(&r.sig);
    }
    out.resize(FILE_LEN, 0);
    Some(out)
}
