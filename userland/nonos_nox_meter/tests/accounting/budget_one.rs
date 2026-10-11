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

//! LISTING_CAP (op 20, v1.4.43): one listing's cap set alone, the global
//! cap, expiry, other listings and what each has spent all kept.

use nonos_nox_meter::budget::{Budget, MAX_LISTING_CAPS};
use nonos_nox_meter::MeterError;
use nonos_payment_proto::{ListingCapAsk, LISTING_CAP_LEN};

fn budget() -> Budget {
    let mut b = Budget::fresh(1_000, 9_000, &[([1; 32], 100), ([2; 32], 200)]).unwrap();
    b.listings[0].spent = 40;
    b.spent = 40;
    b
}

#[test]
fn one_cap_changes_and_the_rest_is_kept() {
    let b = budget().with_listing_cap([1; 32], 500).unwrap();
    assert_eq!((b.cap, b.spent, b.expiry), (1_000, 40, 9_000));
    assert_eq!((b.listings[0].cap, b.listings[0].spent), (500, 40), "spent is kept");
    assert_eq!(b.listings[1].cap, 200);
    let added = budget().with_listing_cap([3; 32], 7).unwrap();
    assert_eq!((added.listings.len(), added.listings[2].cap, added.listings[2].spent), (3, 7, 0));
}

#[test]
fn zero_removes_and_a_full_table_takes_no_new_listing() {
    let gone = budget().with_listing_cap([2; 32], 0).unwrap();
    assert_eq!(gone.listings.len(), 1);
    assert_eq!(budget().with_listing_cap([9; 32], 0).unwrap(), budget(), "nothing to remove");
    let all: Vec<([u8; 32], u128)> = (0..MAX_LISTING_CAPS).map(|i| ([i as u8; 32], 1)).collect();
    let full = Budget::fresh(1, 9, &all).unwrap();
    assert_eq!(full.with_listing_cap([200; 32], 5), Err(MeterError::Full));
    assert!(full.with_listing_cap([0; 32], 5).is_ok(), "a listing already held may change");
}

#[test]
fn the_wire_is_forty_eight_bytes() {
    let a = ListingCapAsk { listing_id: [4; 32], cap: 1 << 100 };
    let b = a.encode();
    assert_eq!((b.len(), ListingCapAsk::parse(&b)), (LISTING_CAP_LEN, Some(a)));
    assert_eq!(ListingCapAsk::parse(&b[..47]), None);
    assert_eq!(ListingCapAsk::parse(&[&b[..], &[0]].concat()), None);
}
