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

//! An order's lines: its state by the escrow's numbers, what waits at the
//! deposit address, and what a delivery's check found.

use nonos_payment_proto::{DeliveredMade, OrderStatus};

use crate::order::delivered::{delivered_line, delivery_due};
use crate::order::rows::{state_name, status_rows};

fn nox(v: u128) -> String {
    format!("{v}")
}

#[test]
fn the_state_and_what_waits_are_read_out() {
    assert_eq!(state_name(1), "funded: waiting for the seller to deliver");
    assert_eq!(state_name(9), "a state this wallet does not know");
    let s = OrderStatus { state: 0, funded: 0, pending: 0, plain: false };
    assert_eq!(status_rows(&s, nox).len(), 2, "nothing waits: no third row");
    let s = OrderStatus { state: 0, funded: 0, pending: 5, plain: false };
    let rows = status_rows(&s, nox);
    assert_eq!(
        rows[2],
        ("waiting", String::from("5 NOX at the deposit address, not yet in the order"))
    );
}

#[test]
fn a_sealed_plaintext_says_so() {
    let s = OrderStatus { state: 2, funded: 9, pending: 0, plain: true };
    let rows = status_rows(&s, nox);
    assert_eq!(rows[2], ("package", String::from("checked and sealed for the install")));
}

#[test]
fn a_delivery_says_ready_or_names_the_bad_chunk() {
    let ok = DeliveredMade::Ok { content_len: 70_000 };
    assert_eq!(delivered_line(&ok), "every chunk checked: 70000 bytes ready to install");
    let bad = DeliveredMade::BadChunk { index: 3, calldata: Vec::new() };
    assert!(delivered_line(&bad).starts_with("chunk 3 is not what was listed"));
}

#[test]
fn an_answered_delivery_is_never_asked_again() {
    let s = |state, plain| OrderStatus { state, funded: 9, pending: 0, plain };
    assert!(delivery_due(Some(&s(2, false)), None), "delivered, not sealed");
    assert!(!delivery_due(Some(&s(1, false)), None), "not delivered yet");
    assert!(!delivery_due(Some(&s(2, true)), None), "already sealed");
    assert!(!delivery_due(None, None), "orderOf not read");
    let bad = DeliveredMade::BadChunk { index: 3, calldata: Vec::new() };
    assert!(!delivery_due(Some(&s(2, false)), Some(&bad)), "a bad chunk waits for the Dispute");
    let ok = DeliveredMade::Ok { content_len: 1 };
    assert!(!delivery_due(Some(&s(2, false)), Some(&ok)));
}
