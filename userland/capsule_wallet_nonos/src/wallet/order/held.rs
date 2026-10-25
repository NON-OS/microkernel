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
 * A channel the listing already has (B8, v1.4.48). Before a voucher
 * listing's form, the meter's ACCOUNT is read a page a tick for a row with
 * the listing's id on its chain, not frozen and not past its end. One found
 * is offered; a second channel opens only on "Open another".
 */

use nonos_payment_proto::{Account, AccountRow};

use super::voucher::Voucher;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Held {
    /// ACCOUNT is read next from this channel.
    Asking(u8),
    /// No open channel for the listing: the form is offered.
    Free,
    Open(AccountRow),
    /// The person chose to open another beside the one open.
    Another,
}

impl Default for Held {
    fn default() -> Self {
        Held::Asking(0)
    }
}

impl Held {
    /// Whether the form to open a channel is offered.
    pub fn offers_form(&self) -> bool {
        matches!(self, Held::Free | Held::Another)
    }
}

/// Whether `r` is an open channel for the voucher's listing at `now` (s).
pub fn open_for(r: &AccountRow, v: &Voucher, now: u64) -> bool {
    (r.listing_id, r.chain_id) == (v.listing_id, v.chain_id) && !r.frozen && r.expiry > now
}

/// Where the search stands after the page from channel `from`. A meter
/// that does not answer offers the form; OPEN then says why it cannot.
pub fn after_page(page: Option<&Account>, from: u8, v: &Voucher, now: u64) -> Held {
    let Some(a) = page else { return Held::Free };
    if let Some(r) = a.rows.iter().find(|r| open_for(r, v, now)) {
        return Held::Open(*r);
    }
    let next = from as usize + a.rows.len();
    match !a.rows.is_empty() && next < a.total as usize {
        true => Held::Asking(next as u8),
        false => Held::Free,
    }
}
