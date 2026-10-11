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

//! A voucher listing that already has an open channel (v1.4.48): ACCOUNT's
//! rows carry listing_id; one open (not frozen, not past its end) is
//! offered, and the form to open another only when the person asks.

use nonos_payment_proto::{Account, AccountRow};

use super::fixture::NOW;
use super::voucher::voucher;
use crate::order::held::{after_page, open_for, Held};

pub(super) fn row(listing_id: [u8; 32], chain_id: u64, expiry: u64, frozen: bool) -> AccountRow {
    let (deposit, settled, cumulative) = (9, 1, 2);
    AccountRow {
        channel_id: [7; 32],
        listing_id,
        chain_id,
        deposit,
        settled,
        cumulative,
        expiry,
        frozen,
        reclaim_to: [0; 20],
    }
}

pub(super) fn page(total: u8, rows: Vec<AccountRow>) -> Account {
    let (cap, spent, expiry, left) = (0, 0, 0, 0);
    Account { flags: 0, address: [0; 20], cap, spent, expiry, left, total, rows }
}

#[test]
fn an_open_channel_for_the_listing_is_offered_and_the_form_is_not() {
    let v = voucher();
    let open = row(v.listing_id, v.chain_id, NOW + 1, false);
    let other = row([0xee; 32], v.chain_id, NOW + 1, false);
    let held = after_page(Some(&page(2, vec![other, open])), 0, &v, NOW);
    assert_eq!(held, Held::Open(open));
    assert!(!held.offers_form(), "a second channel only on Open another");
    assert!(Held::Another.offers_form());
}

#[test]
fn no_open_channel_offers_the_form() {
    let v = voucher();
    let (id, chain) = (v.listing_id, v.chain_id);
    for (r, why) in [
        (row(id, chain, NOW + 1, true), "frozen"),
        (row(id, chain, NOW, false), "at its end"),
        (row(id, chain + 1, NOW + 1, false), "another chain"),
        (row([0xee; 32], chain, NOW + 1, false), "another listing"),
    ] {
        assert!(!open_for(&r, &v, NOW), "{why}");
        assert_eq!(after_page(Some(&page(1, vec![r])), 0, &v, NOW), Held::Free, "{why}");
    }
    assert_eq!(after_page(None, 0, &v, NOW), Held::Free, "a meter that did not answer");
    assert_eq!(after_page(Some(&page(0, vec![])), 0, &v, NOW), Held::Free);
    assert!(Held::Free.offers_form() && !Held::Asking(0).offers_form());
    assert_eq!(Held::default(), Held::Asking(0));
}
