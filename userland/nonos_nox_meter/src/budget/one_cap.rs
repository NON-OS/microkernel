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

//! LISTING_CAP (op 20, the plan, v1.4.43): one listing's cap set alone. The
//! global cap, the expiry and every other listing are kept, and so is what
//! that listing has spent, so a new cap is never a fresh allowance.

use super::caps::Budget;
use super::listing::{ListingCap, MAX_LISTING_CAPS};
use crate::error::MeterError;

impl Budget {
    /// This budget with `listing_id` held to `cap`; a cap of 0 removes it.
    /// A new listing past MAX_LISTING_CAPS is Full.
    pub fn with_listing_cap(&self, listing_id: [u8; 32], cap: u128) -> Result<Budget, MeterError> {
        let mut b = self.clone();
        match b.listings.iter().position(|c| c.listing_id == listing_id) {
            Some(i) if cap == 0 => {
                b.listings.remove(i);
            }
            Some(i) => b.listings[i].cap = cap,
            None if cap == 0 => {}
            None if b.listings.len() >= MAX_LISTING_CAPS => return Err(MeterError::Full),
            None => b.listings.push(ListingCap { listing_id, cap, spent: 0 }),
        }
        Ok(b)
    }
}
