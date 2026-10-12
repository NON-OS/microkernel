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

//! SET_BUDGET: cap u128 | expiry u64 | n u8 (at most 32)
//! | n x (listing_id[32] | cap u128). The reply has no body.

use alloc::vec::Vec;

use crate::bytes::len_u8;
use crate::cursor::Cursor;

mod listing_cap;
pub use listing_cap::{ListingCapAsk, LISTING_CAP_LEN};

pub const MAX_LISTING_CAPS: usize = 32;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SetBudget {
    pub cap: u128,
    pub expiry: u64,
    pub listings: Vec<([u8; 32], u128)>,
}

impl SetBudget {
    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (cap, expiry, n) = (r.u128()?, r.u64()?, r.u8()? as usize);
        if n > MAX_LISTING_CAPS {
            return None;
        }
        let mut listings = Vec::with_capacity(n);
        for _ in 0..n {
            listings.push((r.fixed()?, r.u128()?));
        }
        r.end(SetBudget { cap, expiry, listings })
    }

    /// The body, or None with more caps than the meter keeps.
    pub fn encode(&self) -> Option<Vec<u8>> {
        if self.listings.len() > MAX_LISTING_CAPS {
            return None;
        }
        let mut out = Vec::with_capacity(self.listings.len().saturating_mul(48).saturating_add(25));
        out.extend_from_slice(&self.cap.to_le_bytes());
        out.extend_from_slice(&self.expiry.to_le_bytes());
        out.push(len_u8(self.listings.len()));
        for (id, cap) in &self.listings {
            out.extend_from_slice(id);
            out.extend_from_slice(&cap.to_le_bytes());
        }
        Some(out)
    }
}
