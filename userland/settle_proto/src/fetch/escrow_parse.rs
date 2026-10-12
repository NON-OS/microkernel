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

//! An escrow FETCH read back, exactly: every length checked, every offset
//! checked, nothing past the end.

use alloc::vec::Vec;

use super::escrow::{Escrow, MAX_SOLUTION};
use super::form::MAX_URL;
use crate::bytes::fixed;

impl Escrow {
    /// The body after the form byte, exactly.
    pub fn parse(b: &[u8]) -> Option<Self> {
        let lp = |b: &[u8], max: usize| -> Option<(Vec<u8>, usize)> {
            let n = usize::from(u16::from_le_bytes(fixed(b, 0)?));
            let end = n.checked_add(2)?;
            (n <= max).then(|| b.get(2..end).map(|v| (v.to_vec(), end)))?
        };
        let chain_id = u64::from_le_bytes(fixed(b, 0)?);
        let (url, n) = lp(b.get(8..)?, MAX_URL)?;
        let keys_at = n.checked_add(8)?;
        let key = |i: usize| fixed(b, keys_at.checked_add(i.checked_mul(32)?)?);
        let solution_at = keys_at.checked_add(96)?;
        let (solution, m) = lp(b.get(solution_at..)?, MAX_SOLUTION)?;
        (Some(b.len()) == solution_at.checked_add(m)).then_some(())?;
        let (listing_key, release_key, buyer_pk) = (key(0)?, key(1)?, key(2)?);
        Some(Escrow { chain_id, url, listing_key, release_key, buyer_pk, solution })
    }
}
