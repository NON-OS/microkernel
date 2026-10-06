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

//! The output file onto the data volume. The volume keeps a streamed file only
//! when its bytes hash to the SHA-256 named at the start, so the digest is
//! taken first, with `CryptoHash`, and names the file too: two proofs never
//! share a name, and the same proof saved twice is the same file.

use alloc::string::String;

use nonos_libc::crypto_hash;
use nonos_libc::data::{mk_data_feed, mk_data_feed_begin, mk_data_feed_end};

use super::text::hex;

const SHA256: u64 = 1;
const EALREADY: i64 = -114;
const EIO: i64 = -5;

/// The name the file was saved under, or the negative errno that refused it.
pub fn save(bytes: &[u8]) -> Result<String, i64> {
    let mut sha = [0u8; 32];
    let n = crypto_hash(SHA256, bytes.as_ptr(), bytes.len(), sha.as_mut_ptr(), sha.len());
    if n != 32 {
        return Err(if n < 0 { n } else { EIO });
    }
    let name = alloc::format!("/proof-{}", hex(&sha[..8]));
    let from = match mk_data_feed_begin(name.as_bytes(), &sha, bytes.len() as u64, false) {
        EALREADY => return Ok(name),
        at if at < 0 => return Err(at),
        at => at as usize,
    };
    let rest = bytes.get(from..).ok_or(EIO)?;
    let fed = mk_data_feed(rest);
    if fed < 0 {
        /* Not kept: a stream ended unfinished is dropped, not saved for later. */
        let _ = mk_data_feed_end(false);
        return Err(fed);
    }
    let kept = mk_data_feed_end(false);
    if kept < 0 {
        return Err(kept);
    }
    Ok(name)
}
