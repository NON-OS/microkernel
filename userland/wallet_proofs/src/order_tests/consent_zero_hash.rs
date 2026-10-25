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

//! A keccak the crypto service did not give is all zeros (publish::Kec).
//! An id the wallet computed as all zeros is a refusal of its own, before
//! any compare, so a market answering zero ids never matches a failed hash.

use core::cell::Cell;
use core::ops::Range;

use nonos_marketplace_abi::{release_id, Keccak, SettleMode};
use nonos_payment_proto::Names;

use super::deposit_fixture::K;
use super::gate_fixture::names;
use super::ids_fixture::commerce_for;
use super::market_drive::{market_for, read_terms_with};
use super::read_fixture::escrow;
use crate::order::buy_gate::buyable;
use crate::order::fees_kind::Fees;
use crate::order::pending_step::Step;
use crate::order::terms::asked::UNHASHED;

/// Keccak on the host, but zeros for the calls in `zero`: the service failing.
struct Failing {
    zero: Range<u32>,
    calls: Cell<u32>,
}

impl Keccak for Failing {
    fn keccak256(&self, d: &[u8]) -> [u8; 32] {
        let n = self.calls.replace(self.calls.get() + 1);
        if self.zero.contains(&n) {
            [0; 32]
        } else {
            K.keccak256(d)
        }
    }
}

fn refused(zero: Range<u32>, n: &Names, ids: ([u8; 32], [u8; 32])) {
    let k = Failing { zero, calls: Cell::new(0) };
    let mut c = commerce_for(n, SettleMode::Escrow);
    (c.listing_id, c.release_id) = ids;
    let read = read_terms_with(&k, n, &market_for(n, &c, &escrow())).1;
    assert_eq!(read.as_ref().err(), Some(&UNHASHED));
    let read = read.map(|t| t.named);
    assert!(buyable(&Step::Consent, &read, &Fees::Quoted("0.01".into())).is_none());
}

#[test]
fn zero_ids_from_the_market_never_match_a_failed_hash() {
    let (n, z) = (names(), [0; 32]);
    refused(0..u32::MAX, &n, (z, z));
    // listingId is two calls, then releaseId two: each failing alone.
    let listing = commerce_for(&n, SettleMode::Escrow).listing_id;
    refused(2..4, &n, (listing, z));
    let version = core::str::from_utf8(&n.release).unwrap();
    refused(0..2, &n, (z, release_id(&K, &z, version)));
}
