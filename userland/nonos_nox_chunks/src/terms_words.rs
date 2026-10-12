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

//! The fourteen words back into OrderTerms, strictly: what the meter keeps
//! of an order is its words, and a dispute's calldata needs the terms. A word
//! with bits past its field's width is refused, so words() of the result is
//! the input again.

use crate::terms::{OrderTerms, TERMS_WORDS};

fn tail<const N: usize>(w: &[u8; 32]) -> Option<[u8; N]> {
    let at = 32usize.checked_sub(N)?;
    w.get(..at)?.iter().all(|b| *b == 0).then_some(())?;
    w.get(at..)?.try_into().ok()
}

impl OrderTerms {
    pub fn from_words(w: &[[u8; 32]; TERMS_WORDS]) -> Option<OrderTerms> {
        Some(OrderTerms {
            listing_id: w[0],
            release_id: w[1],
            bond_id: w[2],
            payee: tail(&w[3])?,
            token: tail(&w[4])?,
            amount: u128::from_be_bytes(tail(&w[5])?),
            delivery_deadline: u64::from_be_bytes(tail(&w[6])?),
            dispute_window: u64::from_be_bytes(tail(&w[7])?),
            chunk_root: w[8],
            content_len: u64::from_be_bytes(tail(&w[9])?),
            cipher_root: w[10],
            key_commit: w[11],
            refund_commit: w[12],
            salt: w[13],
        })
    }
}
