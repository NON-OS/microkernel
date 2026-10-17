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

//! An order whose delivery has a bad chunk, and the meter's dispute() for it
//! built by nonos_nox_chunks itself.

use nonos_nox_chunks::{dispute_call, Dispute, OrderTerms};
use nonos_payment_proto::{DeliveredMade, OrderMade};
use nonos_settle_proto::Quote;

use super::fixture::{buy, made, offer, NOW, REFUND_PAIR};

pub(super) const AT: u64 = NOW + 86_400;

pub(super) fn order() -> (OrderMade, DeliveredMade) {
    let m = made(&buy().order_new(&offer(), REFUND_PAIR, NOW));
    let t = OrderTerms::from_words(&m.terms).unwrap();
    let d = Dispute {
        index: 3,
        cipher_chunk: vec![0x5a; 1024],
        plain_hash: [0x11; 32],
        cipher_proof: vec![[0x22; 32]; 7],
        leaf_proof: vec![[0x33; 32]; 7],
    };
    (m, DeliveredMade::BadChunk { index: 3, calldata: dispute_call(&t, &d).unwrap() })
}

pub(super) const ESCROW: [u8; 20] = [0xE5; 20];
pub(super) const GWEI: u128 = 1_000_000_000;

pub(super) fn word(n: u64) -> [u8; 32] {
    let mut w = [0; 32];
    w[24..].copy_from_slice(&n.to_be_bytes());
    w
}

pub(super) fn quote() -> Quote {
    let (gas_limit, max_fee, max_priority) = (300_000, 20 * GWEI, GWEI);
    Quote { kind: 10, to: ESCROW, nonce: 0, gas_limit, max_fee, max_priority }
}
