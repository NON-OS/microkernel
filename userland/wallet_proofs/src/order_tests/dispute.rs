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

//! A bad chunk disputed (B8, kind 10, v1.4.49): the meter's calldata goes
//! as it came, from refundTo alone, and only inside the window.

use nonos_nox_config::{DELIVER_MARGIN, DISPUTE_WINDOW};
use nonos_payment_proto::DeliveredMade;

use super::dispute_fixture::{order, AT};
use super::fixture::REFUND_TO;
use crate::order::dispute::dispute;
use crate::order::dispute_said::*;
use crate::order::send_step::EscrowSend;

#[test]
fn the_meters_calldata_goes_through_untouched_from_refund_to() {
    let (m, bad) = order();
    let DeliveredMade::BadChunk { calldata, .. } = &bad else { unreachable!() };
    let got = dispute(&m, &bad, REFUND_TO, REFUND_TO, (2, AT), AT + 60).unwrap();
    assert_eq!(got, EscrowSend { kind: 10, from: REFUND_TO, calldata: calldata.clone() });
    let ok = DeliveredMade::Ok { content_len: 1 };
    assert_eq!(dispute(&m, &ok, REFUND_TO, REFUND_TO, (2, AT), AT), Err(NOTHING));
}

#[test]
fn any_address_but_refund_to_is_refused() {
    let (m, bad) = order();
    for from in [[0x99; 20], [0; 20]] {
        assert_eq!(dispute(&m, &bad, REFUND_TO, from, (2, AT), AT), Err(NOT_REFUND_TO));
    }
    assert_eq!(dispute(&m, &bad, [0; 20], [0; 20], (2, AT), AT), Err(NOT_REFUND_TO));
}

#[test]
fn a_dispute_after_the_window_or_of_another_order_is_refused() {
    let (m, bad) = order();
    let end = AT + DISPUTE_WINDOW;
    let at = |now| dispute(&m, &bad, REFUND_TO, REFUND_TO, (2, AT), now);
    assert!(at(end - DELIVER_MARGIN).is_ok(), "exactly the margin left");
    assert_eq!(at(end - DELIVER_MARGIN + 1), Err(TOO_LATE));
    assert_eq!(at(end + 1), Err(TOO_LATE));
    let not_yet = |of| dispute(&m, &bad, REFUND_TO, REFUND_TO, of, AT);
    assert_eq!(not_yet((1, 0)), Err(NOT_DELIVERED));
    assert_eq!(not_yet((2, 0)), Err(NOT_DELIVERED), "no deliveredAt");
    let mut other = m;
    other.terms[13][0] ^= 1;
    let r = dispute(&other, &bad, REFUND_TO, REFUND_TO, (2, AT), AT);
    assert_eq!(r, Err(NOT_THIS_ORDER), "another order's salt");
    let DeliveredMade::BadChunk { index, mut calldata } = bad else { unreachable!() };
    calldata[0] ^= 1;
    let other_call = DeliveredMade::BadChunk { index, calldata };
    let r = dispute(&m, &other_call, REFUND_TO, REFUND_TO, (2, AT), AT);
    assert_eq!(r, Err(NOT_THIS_ORDER), "another selector");
}
