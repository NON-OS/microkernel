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

use nonos_nox_chunks::{buyer_terms, deposit_address, order_id, Buyer, Sale};
use nonos_nox_config::REFUND_TAG;
use nonos_nox_config::{DELIVER_MARGIN, MAX_DELIVERY, ORDER_MIN_LEAD};
use nonos_nox_meter::orders::order_plan;
use nonos_nox_meter::MeterError;
use nonos_payment_proto::OrderNew;

use super::fixture::{new, ESCROW, FWD, NOW, NOX, REFUND_SALT, SALT};
use super::host::{keccak, Host};

#[test]
fn an_order_is_the_buyers_terms_its_id_and_its_deposit_address() {
    let o = new(NOW + ORDER_MIN_LEAD, 1_000);
    let got = order_plan(&Host, &o, (&NOX, &ESCROW, &FWD), SALT, NOW).unwrap();
    let sale = Sale {
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [3; 32],
        payee: [4; 20],
        amount: 1_000,
        chunk_root: [5; 32],
        content_len: 4096,
        cipher_root: [6; 32],
        key_commit: [7; 32],
    };
    let b =
        Buyer { deadline: o.deadline, refund_to: [8; 20], refund_salt: REFUND_SALT, salt: SALT };
    let terms = buyer_terms(&Host, &sale, &NOX, &b, NOW).unwrap();
    let id = order_id(&Host, 11155111, &ESCROW, &terms);
    assert_eq!((got.order_id, got.terms), (id, terms.words()));
    assert_eq!(got.deposit, deposit_address(&Host, &ESCROW, &id, &FWD));
    assert_eq!((got.chain_id, got.refund_to, got.refund_salt), (11155111, [8; 20], REFUND_SALT));
    let mut to = [0u8; 32];
    to[12..].copy_from_slice(&[8; 20]);
    assert_eq!(got.terms[12], keccak(&[REFUND_TAG, to, REFUND_SALT].concat()), "refundCommit");
}

#[test]
fn an_order_outside_the_window_or_empty_is_refused() {
    let plan = |o: OrderNew| order_plan(&Host, &o, (&NOX, &ESCROW, &FWD), SALT, NOW).err();
    assert_eq!(plan(new(NOW + ORDER_MIN_LEAD - 1, 1_000)), Some(MeterError::Expired));
    assert_eq!(
        plan(new(NOW + MAX_DELIVERY - DELIVER_MARGIN + 1, 1_000)),
        Some(MeterError::Expired)
    );
    assert_eq!(plan(new(NOW + MAX_DELIVERY - DELIVER_MARGIN, 1_000)), None);
    assert_eq!(plan(new(NOW + ORDER_MIN_LEAD, 0)), Some(MeterError::Mismatch));
}
