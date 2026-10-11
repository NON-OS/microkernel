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

//! OP_LISTING (v1.4.16). Request: chain_hash(chain_id, listingId). Reply:
//! n u8 | n x (releaseId[32] | releaseHash[32] | bondId[32] | block u64 |
//! uri_len u16 | uri), at most LISTED_MAX rows, newest first, each uri 1 to
//! MAX_URL bytes. Rows are hints: the market checks each by READ 40, FETCH
//! and its own gates.

use alloc::vec::Vec;

use crate::fetch::MAX_URL;
use crate::registry_current::LISTED_MAX;
use crate::registry_log::Listed;

const ROW_HEAD: usize = 32 * 3 + 8 + 2;

/// The reply for `rows`, the first LISTED_MAX whose uri is 1 to MAX_URL
/// bytes: a longer one would not fit its u16 length, and the bytes past
/// the wrapped length would read as rows the log never had.
pub fn listing_reply(rows: &[Listed]) -> Vec<u8> {
    let fits = |r: &&Listed| !r.uri.is_empty() && r.uri.len() <= MAX_URL;
    let rows: Vec<&Listed> = rows.iter().filter(fits).take(LISTED_MAX).collect();
    let mut out = Vec::from([rows.len() as u8]);
    for r in rows {
        [&r.release_id, &r.release_hash, &r.bond_id].iter().for_each(|w| out.extend_from_slice(*w));
        out.extend_from_slice(&r.block.to_le_bytes());
        out.extend_from_slice(&(r.uri.len() as u16).to_le_bytes());
        out.extend_from_slice(&r.uri);
    }
    out
}

/// The rows of a reply to the ask for `listing_id`, or None unless whole.
pub fn parse_listing_reply(b: &[u8], listing_id: &[u8; 32]) -> Option<Vec<Listed>> {
    let (&n, mut rest) = b.split_first()?;
    if n as usize > LISTED_MAX {
        return None;
    }
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let head = rest.get(..ROW_HEAD)?;
        let len = u16::from_le_bytes(head[104..106].try_into().ok()?) as usize;
        let uri =
            rest.get(ROW_HEAD..ROW_HEAD + len).filter(|u| !u.is_empty() && u.len() <= MAX_URL)?;
        out.push(Listed {
            listing_id: *listing_id,
            release_id: head[..32].try_into().ok()?,
            release_hash: head[32..64].try_into().ok()?,
            bond_id: head[64..96].try_into().ok()?,
            block: u64::from_le_bytes(head[96..104].try_into().ok()?),
            uri: uri.to_vec(),
        });
        rest = &rest[ROW_HEAD + len..];
    }
    rest.is_empty().then_some(out)
}
