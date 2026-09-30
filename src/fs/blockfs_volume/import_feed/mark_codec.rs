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
 * A streamed import's mark as bytes: the digest and length it must reach,
 * the hash so far and the writer's state. It only resumes the stream it
 * names, whole.
 */

use alloc::vec::Vec;

use super::hash::PinHash;
use super::hash_save::HASH_STATE_BYTES;
use crate::fs::blockfs::FileStream;

const MAGIC: [u8; 8] = *b"NONOSPT1";
const HEAD: usize = 8 + 32 + 8;

pub(super) fn encode(want: &[u8; 32], bytes: u64, hash: &PinHash, stream: &FileStream) -> Vec<u8> {
    let mut out = [&MAGIC[..], &want[..], &bytes.to_le_bytes()[..]].concat();
    hash.save(&mut out);
    out.extend_from_slice(&stream.save());
    out
}

/* The hash and writer in `b`, when it is a whole mark for `want` and `bytes`. */
pub(super) fn decode(b: &[u8], want: &[u8; 32], bytes: u64) -> Option<(PinHash, FileStream)> {
    let at = HEAD + HASH_STATE_BYTES;
    if b.len() <= at || b[..8] != MAGIC || &b[8..40] != want || b[40..48] != bytes.to_le_bytes() {
        return None;
    }
    let hash = PinHash::load(b[HEAD..at].try_into().ok()?);
    let stream = FileStream::load(&b[at..])?;
    (hash.taken() == stream.size() && stream.size() <= bytes).then_some((hash, stream))
}
