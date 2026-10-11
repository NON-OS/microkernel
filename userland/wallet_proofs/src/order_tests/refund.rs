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

//! A refund from refundTo (B8, kind 11, v1.4.51): refund(T, refundTo,
//! refundSalt) built from the order's own terms, sent from refundTo alone,
//! quoted under REFUND_GAS.

use nonos_nox_config::REFUND_GAS;
use nonos_settle_proto::Quote;

use super::dispute_fixture::{quote, word, ESCROW};
use super::fixture::{buy, made, offer, NOW, REFUND_TO};
use crate::order::refund_call::{refund_call, refunded, state_and_extra};
use crate::order::send_gas::after_quote;
use crate::order::send_step::{EscrowSend, SendStep};

pub(super) const SALT: [u8; 32] = [0x5a; 32];

#[test]
fn the_refund_call_is_t_then_refund_to_then_the_salt() {
    let m = made(&buy().order_new(&offer(), REFUND_TO, NOW));
    let c = refund_call(&m, &REFUND_TO, &SALT).unwrap();
    assert_eq!((c.len(), &c[..4]), (4 + 16 * 32, &[0xa3, 0x81, 0x4a, 0x14][..]));
    assert_eq!(&c[4..452], &m.terms.concat()[..]);
    assert_eq!(
        (&c[452..464], &c[464..484], &c[484..]),
        (&[0u8; 12][..], &REFUND_TO[..], &SALT[..])
    );
}

#[test]
fn order_of_reads_state_and_extra_and_refunded_is_state_5() {
    let mut extra = word(0);
    extra[16..].copy_from_slice(&7u128.to_be_bytes());
    assert_eq!(state_and_extra(&[word(4), word(9), extra, word(0), word(0)]), Some((4, 7)));
    assert_eq!(state_and_extra(&[word(4), word(9), [0xff; 32], word(0), word(0)]), None);
    assert_eq!(state_and_extra(&[word(4); 4]), None);
    assert!(refunded(Some(&[word(5), word(0), word(0), word(0), word(0)])));
    assert!(!refunded(Some(&[word(3), word(0), word(0), word(0), word(0)])) && !refunded(None));
}

#[test]
fn a_refunds_quote_is_held_under_refund_gas() {
    let send = EscrowSend { kind: 11, from: REFUND_TO, calldata: vec![0xa3, 0x81, 0x4a, 0x14] };
    let q = Quote { kind: 11, gas_limit: REFUND_GAS, ..quote() };
    let ok = after_quote(send.clone(), q, Some(ESCROW), u128::MAX, 11_155_111);
    let SendStep::Signing(ask) = ok else { panic!("not signing") };
    assert_eq!(
        (ask.address, ask.to, ask.gas, &ask.data),
        (REFUND_TO, ESCROW, REFUND_GAS, &send.calldata)
    );
    let over = Quote { gas_limit: REFUND_GAS + 1, ..q };
    assert!(matches!(after_quote(send.clone(), over, Some(ESCROW), 0, 1), SendStep::Failed(_)));
    let dispute_quote = quote();
    assert!(matches!(after_quote(send, dispute_quote, Some(ESCROW), 0, 1), SendStep::Failed(_)));
}
