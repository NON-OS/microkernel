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

//! When a refund is sent (plan/27): Disputed; Funded, or NOX waiting at an
//! unfunded deposit address, from deliveryDeadline on; extra after Released
//! or Refunded; and only to the refundTo drawn at Buy.

use super::fixture::{buy, made, offer, NOW, REFUND_TO};
use super::refund::SALT;
use crate::order::dispute_said::NOT_REFUND_TO;
use crate::order::refund::refund;
use crate::order::refund_said::*;

#[test]
fn each_state_refunds_or_says_why_not() {
    let m = made(&buy().order_new(&offer(), REFUND_TO, NOW));
    let deadline = u64::from_be_bytes(m.terms[6][24..].try_into().unwrap());
    let at = |of, pending, now| refund(&m, REFUND_TO, (REFUND_TO, SALT), of, pending, now);
    assert!(at((3, 0), 0, NOW).is_ok(), "disputed, at once");
    assert_eq!(at((1, 0), 0, deadline - 1), Err(NOT_YET));
    assert!(at((1, 0), 0, deadline).is_ok(), "funded, from the deadline");
    assert_eq!(at((0, 0), 5, deadline - 1), Err(NOT_YET));
    assert!(at((0, 0), 5, deadline).is_ok(), "NOX waiting at an unfunded deposit");
    assert_eq!(at((0, 0), 0, deadline), Err(NOTHING_LEFT));
    assert_eq!(at((2, 0), 0, deadline), Err(IN_WINDOW));
    assert_eq!(at((4, 0), 0, deadline), Err(NOTHING_LEFT));
    assert!(at((4, 3), 0, NOW).is_ok(), "extra after Released");
    assert_eq!(at((5, 0), 0, deadline), Err(ALREADY));
    assert!(at((5, 3), 0, NOW).is_ok(), "extra after Refunded");
    let sent = at((3, 0), 0, NOW).unwrap();
    assert_eq!((sent.kind, sent.from, sent.calldata.len()), (11, REFUND_TO, 516));
}

#[test]
fn only_the_refund_to_drawn_at_buy_sends_it() {
    let m = made(&buy().order_new(&offer(), REFUND_TO, NOW));
    let r = |drawn, answered| refund(&m, drawn, (answered, SALT), (3, 0), 0, NOW);
    assert_eq!(r(REFUND_TO, [0x99; 20]), Err(NOT_REFUND_TO));
    assert_eq!(r([0; 20], [0; 20]), Err(NOT_REFUND_TO));
}
