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
use alloc::vec::Vec;

use super::caps::Budget;
use super::listing::{ListingCap, MAX_LISTING_CAPS};
use crate::error::MeterError;

impl Budget {
    /// A new allowance from the owner: `cap` in all, `listings` caps for
    /// single listings, until `expiry`. It starts with nothing spent, so each
    /// SET_BUDGET grants its cap afresh. More caps than the file holds, or one
    /// listing named twice, is refused rather than half applied.
    pub fn fresh(
        cap: u128,
        expiry: u64,
        listings: &[([u8; 32], u128)],
    ) -> Result<Budget, MeterError> {
        if listings.len() > MAX_LISTING_CAPS {
            return Err(MeterError::Full);
        }
        let mut caps: Vec<ListingCap> = Vec::with_capacity(listings.len());
        for &(listing_id, cap) in listings {
            if caps.iter().any(|c| c.listing_id == listing_id) {
                return Err(MeterError::Duplicate);
            }
            caps.push(ListingCap { listing_id, cap, spent: 0 });
        }
        Ok(Budget { cap, spent: 0, expiry, listings: caps })
    }

    /// What may still be spent at `now`: zero once expired.
    pub fn left(&self, now: u64) -> u128 {
        if now >= self.expiry {
            return 0;
        }
        self.cap.saturating_sub(self.spent)
    }
}
