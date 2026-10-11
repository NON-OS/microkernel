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
 * Reading the ReleaseAttest log back: exactly FILE_LEN bytes, this magic
 * and version, every used slot a record, every unused byte zero, or
 * nothing.
 */

use alloc::vec::Vec;

use super::file::{FILE_LEN, MAGIC, MAX_ATTESTS, RECORD_LEN};
use super::types::{Attest, Record};

fn record(b: &[u8]) -> Option<Record> {
    let w = |at: usize| -> Option<[u8; 32]> { b.get(at..at + 32)?.try_into().ok() };
    let attest = Attest {
        chain_id: u64::from_le_bytes(b.get(..8)?.try_into().ok()?),
        listing_id: w(8)?,
        release_id: w(40)?,
        bond_id: w(72)?,
        chunk_root: w(104)?,
        content_len: u64::from_le_bytes(b.get(136..144)?.try_into().ok()?),
        package_hash: w(144)?,
    };
    let sig: [u8; 65] = b.get(176..241)?.try_into().ok()?;
    /* A used slot names a chain and carries a signature: an empty one read
     * as used would answer for a release nothing was signed for. */
    (attest.chain_id != 0 && sig != [0; 65]).then_some(Record { attest, sig })
}

pub fn decode(b: &[u8]) -> Option<Vec<Record>> {
    let header_ok = b.len() == FILE_LEN && &b[..4] == MAGIC && b[4..6] == 1u16.to_le_bytes();
    let n = u16::from_le_bytes(b.get(6..8)?.try_into().ok()?) as usize;
    if !header_ok || n > MAX_ATTESTS || b[8 + n * RECORD_LEN..].iter().any(|x| *x != 0) {
        return None;
    }
    b[8..8 + n * RECORD_LEN].chunks_exact(RECORD_LEN).map(record).collect()
}
