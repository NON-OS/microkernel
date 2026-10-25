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

use super::fixture::{start, SETTLEMENT};
use super::host::keccak;
use nonos_nox_meter::open::{approve_data, open_data, APPROVE_SELECTOR, OPEN_SELECTOR};

#[test]
fn the_selectors_are_their_signatures() {
    assert_eq!(APPROVE_SELECTOR, keccak(b"approve(address,uint256)")[..4]);
    assert_eq!(OPEN_SELECTOR, keccak(b"open(address,bytes32,uint256,uint64,address)")[..4]);
}

#[test]
fn approve_and_open_are_abi_encode_words() {
    let s = start();
    let a = approve_data(&SETTLEMENT, s.amount);
    assert_eq!((&a[4..16], &a[16..36]), (&[0u8; 12][..], &SETTLEMENT[..]));
    assert_eq!(&a[36..], &[[0u8; 30].as_slice(), &1_000u16.to_be_bytes()].concat()[..]);
    let o = open_data(&s.payee, &s.listing_id, s.amount, s.expiry, &s.reclaim_to);
    assert_eq!(&o[16..36], &s.payee);
    assert_eq!(&o[36..68], &s.listing_id);
    assert_eq!(&o[92..100], &[0, 0, 0, 0, 0, 0, 0x03, 0xe8]);
    assert_eq!(&o[124..132], &s.expiry.to_be_bytes());
    assert_eq!((&o[132..144], &o[144..]), (&[0u8; 12][..], &s.reclaim_to[..]));
}
