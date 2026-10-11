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

//! A delivery's body must be this record's, this supervisor's and the
//! buyer's own; its device proof must be bound to ctx2 under e2 on an
//! accepted day; its measurements must rebuild M.

use nonos_proof_abi::{verify_delivery, Delivery, Refusal};

use crate::fixture::{delivered, delivery_expected, DAY, NONCE, REQUEST};

#[test]
fn a_body_not_the_buyers_own_is_refused() {
    let d = delivered();
    let pinned = d.pinned();
    let pins = pinned.trust();
    let ex = delivery_expected(&d.policy, &pins);
    let other_nonce = verify_delivery(&d.bytes(), &delivery_expected_with(&ex, [1; 32], REQUEST));
    assert_eq!(other_nonce.err(), Some(Refusal::Body), "another nonce");
    let other_req = verify_delivery(&d.bytes(), &delivery_expected_with(&ex, NONCE, b"other"));
    assert_eq!(other_req.err(), Some(Refusal::Body), "another request");
    let with = |b: Delivery| d.bytes_with(&d.record.encode(), &b.encode(), &d.meas.concat());
    let refused = |b: Delivery| verify_delivery(&with(b), &ex).err();
    assert_eq!(refused(Delivery { service_meas: [3; 32], ..d.body }), Some(Refusal::Body));
    assert_eq!(refused(Delivery { record_id: [3; 32], ..d.body }), Some(Refusal::Body));
    assert_eq!(refused(Delivery { order_ref: [3; 32], ..d.body }), Some(Refusal::Body));
    assert_eq!(refused(Delivery { channel_ref: [3; 32], ..d.body }), Some(Refusal::Body));
    let moved = refused(Delivery { from: 101, ..d.body });
    assert_eq!(moved, Some(Refusal::DeviceBinding), "another body is another ctx2");
}

fn delivery_expected_with<'a>(
    ex: &nonos_proof_abi::DeliveryExpected<'a>,
    nonce: [u8; 32],
    request: &'a [u8],
) -> nonos_proof_abi::DeliveryExpected<'a> {
    nonos_proof_abi::DeliveryExpected { nonce, request, ..*ex }
}

#[test]
fn the_scope_the_day_and_the_measurements_are_held_to() {
    let d = delivered();
    let pinned = d.pinned();
    let pins = pinned.trust();
    let ex = delivery_expected(&d.policy, &pins);
    let other_listing = nonos_proof_abi::DeliveryExpected { listing_id: [0x4D; 32], ..ex };
    let r = verify_delivery(&d.bytes(), &other_listing).err();
    assert_eq!(r, Some(Refusal::DeviceTrust), "e2 is the listing's");
    let later = nonos_proof_abi::DeliveryExpected { days: (DAY + 1, DAY + 2), ..ex };
    assert_eq!(verify_delivery(&d.bytes(), &later).err(), Some(Refusal::Window));
    let short = d.bytes_with(&d.record.encode(), &d.body.encode(), &d.meas[..1].concat());
    assert_eq!(verify_delivery(&short, &ex).err(), Some(Refusal::Measurements));
}
