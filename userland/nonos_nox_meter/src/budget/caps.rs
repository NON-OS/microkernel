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

//! The owner's consent, set from the wallet: a global cap, optional caps per
//! listing, and an expiry. The default is no budget at all, so the meter
//! charges nothing until the owner has said how much it may.

use alloc::vec::Vec;

use super::add::add_within;
use super::listing::ListingCap;
use crate::error::MeterError;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Budget {
    pub cap: u128,
    pub spent: u128,
    /// Unix seconds; zero is the default and means nothing is allowed.
    pub expiry: u64,
    pub listings: Vec<ListingCap>,
}

impl Budget {
    /// The budget after spending `amount` on `listing`, or why it may not.
    pub fn spending(
        &self,
        listing: &[u8; 32],
        amount: u128,
        now: u64,
    ) -> Result<Budget, MeterError> {
        if now >= self.expiry {
            return Err(MeterError::Expired);
        }
        let mut next = self.clone();
        next.spent = add_within(next.spent, amount, next.cap)?;
        if let Some(l) = next.listings.iter_mut().find(|l| &l.listing_id == listing) {
            l.spent = add_within(l.spent, amount, l.cap)?;
        }
        Ok(next)
    }

    /// Spend the chain says happened, counted whether or not it fits: the
    /// budget can show an overdraft, it cannot hide one.
    pub fn absorb(&mut self, listing: &[u8; 32], amount: u128) {
        self.spent = self.spent.saturating_add(amount);
        if let Some(l) = self.listings.iter_mut().find(|l| &l.listing_id == listing) {
            l.spent = l.spent.saturating_add(amount);
        }
    }
}
