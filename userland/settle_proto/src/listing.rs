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

use crate::bytes::{fixed, len_u16, len_u8};
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
    let mut out = Vec::from([len_u8(rows.len())]);
    for r in rows {
        [&r.release_id, &r.release_hash, &r.bond_id].iter().for_each(|w| out.extend_from_slice(*w));
        out.extend_from_slice(&r.block.to_le_bytes());
        out.extend_from_slice(&len_u16(r.uri.len()).to_le_bytes());
        out.extend_from_slice(&r.uri);
    }
    out
}

/// The rows of a reply to the ask for `listing_id`, or None unless whole.
pub fn parse_listing_reply(b: &[u8], listing_id: &[u8; 32]) -> Option<Vec<Listed>> {
    let (&n, mut rest) = b.split_first()?;
    if usize::from(n) > LISTED_MAX {
        return None;
    }
    let mut out = Vec::with_capacity(usize::from(n));
    for _ in 0..n {
        let len = usize::from(u16::from_le_bytes(fixed(rest, 104)?));
        let end = ROW_HEAD.checked_add(len)?;
        let uri = rest.get(ROW_HEAD..end).filter(|u| !u.is_empty() && u.len() <= MAX_URL)?;
        out.push(Listed {
            listing_id: *listing_id,
            release_id: fixed(rest, 0)?,
            release_hash: fixed(rest, 32)?,
            bond_id: fixed(rest, 64)?,
            block: u64::from_le_bytes(fixed(rest, 96)?),
            uri: uri.to_vec(),
        });
        rest = rest.get(end..)?;
    }
    rest.is_empty().then_some(out)
}
