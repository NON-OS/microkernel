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

//! ORDER_NEW's refundCommit is the request's refundTo under the request's
//! refundSalt, never a salt the meter draws; a request without one is
//! refused, by the exact-length wire or as a zero salt.

use nonos_nox_config::{ORDER_MIN_LEAD, REFUND_TAG};
use nonos_nox_meter::orders::order_plan;
use nonos_nox_meter::MeterError;
use nonos_payment_proto::OrderNew;

use super::fixture::{new, ESCROW, FWD, NOW, NOX, SALT};
use super::host::{keccak, Host};

fn commit(to: [u8; 20], salt: [u8; 32]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[12..].copy_from_slice(&to);
    keccak(&[REFUND_TAG, w, salt].concat())
}

fn plan(o: &OrderNew, salt: [u8; 32]) -> Result<nonos_nox_meter::orders::Order, MeterError> {
    order_plan(&Host, o, (&NOX, &ESCROW, &FWD), salt, NOW)
}

#[test]
fn the_plan_commits_the_requests_refund_to_under_the_requests_salt() {
    let base = new(NOW + ORDER_MIN_LEAD, 1_000);
    let pairs = [([0x21; 20], [0x31; 32]), ([0x22; 20], [0x31; 32]), ([0x21; 20], [0x32; 32])];
    for (to, salt) in pairs {
        let o = OrderNew { refund_to: to, refund_salt: salt, ..base };
        let got = plan(&o, SALT).unwrap();
        assert_eq!(got.terms[12], commit(to, salt), "refundCommit over the request's pair");
        assert_eq!((got.refund_to, got.refund_salt), (to, salt), "kept for ORDER_REFUND");
        assert_eq!(got.terms[13], SALT, "the order salt stays the meter's");
    }
    let other = plan(&base, [0x99; 32]).unwrap();
    assert_eq!(other.terms[12], plan(&base, SALT).unwrap().terms[12], "not the meter's salt");
    assert_ne!(other.order_id, plan(&base, SALT).unwrap().order_id, "its salt is still in the id");
}

#[test]
fn a_request_without_a_refund_salt_is_refused() {
    let o = new(NOW + ORDER_MIN_LEAD, 1_000);
    let b = o.encode();
    assert_eq!(OrderNew::parse(&b), Some(o));
    assert_eq!(OrderNew::parse(&b[..b.len() - 32]), None, "the body without refundSalt");
    let zero = OrderNew { refund_salt: [0; 32], ..o };
    assert_eq!(plan(&zero, SALT).err(), Some(MeterError::Mismatch), "a zero refundSalt");
    assert_eq!(OrderNew::parse(&zero.encode()).map(|z| z.refund_salt), Some([0; 32]));
}
