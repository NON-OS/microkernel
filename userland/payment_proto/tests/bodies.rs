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

//! Each body round-trips, and every cut or tail is refused.

use nonos_payment_proto::*;

mod common;
use common::refused_cut_and_tail;

fn pay() -> PayRequest<'static> {
    PayRequest {
        listing_id: [1; 32],
        release_hash: [2; 32],
        payee: [3; 20],
        unit: [4; 32],
        qty: 5,
        unit_price: 6,
        call_id: [7; 32],
        chain_id: 11_155_111,
        listing: b"linux.qwen-small",
        release: b"qwen-small@1",
    }
}

#[test]
fn pay_round_trips_with_u16_strings_and_refuses_the_rest() {
    let b = pay().encode().unwrap();
    assert_eq!(b.len(), 180 + 2 + 16 + 2 + 12);
    assert_eq!(&b[180..182], &16u16.to_le_bytes());
    assert_eq!(PayRequest::parse(&b), Some(pay()));
    refused_cut_and_tail(&b, |b| PayRequest::parse(b).map(|_| ()));
    let empty = PayRequest { listing: b"", ..pay() }.encode().unwrap();
    assert_eq!(PayRequest::parse(&empty), None);
    let long = [b'x'; 70_000];
    assert_eq!(PayRequest { listing: &long, ..pay() }.encode(), None);
}

#[test]
fn set_budget_round_trips_up_to_32_caps() {
    let s = SetBudget { cap: 500, expiry: 9, listings: vec![([1; 32], 40), ([2; 32], 60)] };
    let b = s.encode().unwrap();
    assert_eq!(b.len(), 25 + 2 * 48);
    assert_eq!(SetBudget::parse(&b), Some(s.clone()));
    refused_cut_and_tail(&b, SetBudget::parse);
    let many = SetBudget { listings: vec![([0; 32], 1); 33], ..s };
    assert_eq!(many.encode(), None);
}
