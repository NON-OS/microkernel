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

//! A dispute sent from the wallet, step by step (B8, kind 10, v1.4.51):
//! orderOf, refundTo, the quote, the gas from the Shield, the signature.

use nonos_nox_config::{DISPUTE_GAS, DISPUTE_WINDOW};
use nonos_payment_proto::DeliveredMade;

use super::dispute_fixture::{order, quote, word, AT, ESCROW, GWEI};
use super::fixture::REFUND_TO;

const SEPOLIA: u64 = 11_155_111;
use crate::order::dispute_said::{NOT_REFUND_TO, TOO_LATE};
use crate::order::dispute_steps::{landed, Disputed};
use crate::order::send_gas::{after_balance, after_quote};
use crate::order::send_step::SendStep;

#[test]
fn a_dispute_goes_from_order_of_to_its_signature() {
    let (m, bad) = order();
    let d = Disputed { made: &m, delivered: &bad, refund_to: REFUND_TO };
    let of = [word(2), word(9), word(0), word(5), word(AT)];
    assert_eq!(d.after_read(Some(&of), AT + 60), SendStep::RefundTo((2, AT)));
    let SendStep::Quoting(send) = d.after_refund_to(REFUND_TO, (2, AT), AT + 60) else {
        panic!("not quoting")
    };
    let DeliveredMade::BadChunk { calldata, .. } = &bad else { unreachable!() };
    assert_eq!((send.from, &send.calldata), (REFUND_TO, calldata));
    let gas = after_quote(send.clone(), quote(), Some(ESCROW), 0, SEPOLIA);
    let size = 10_000_000_000_000_000;
    assert_eq!(gas, SendStep::Gas { send: send.clone(), quote: quote(), size });
    let cost = 300_000 * 20 * GWEI;
    assert_eq!(after_balance(gas.clone(), cost - 1, SEPOLIA), gas, "the withdraw has not landed");
    let SendStep::Signing(ask) = after_balance(gas, cost, SEPOLIA) else { panic!("not signing") };
    assert_eq!((ask.address, ask.to, ask.value, ask.chain_id), (REFUND_TO, ESCROW, 0, SEPOLIA));
    assert_eq!((ask.gas, ask.max_fee, &ask.data), (300_000, 20 * GWEI, calldata));
    let paid = after_quote(send, quote(), Some(ESCROW), cost, SEPOLIA);
    assert!(matches!(paid, SendStep::Signing(_)), "gas already there");
    assert!(landed(Some(&[word(3), word(9), word(0), word(5), word(AT)])));
    assert!(!landed(Some(&of)) && !landed(None));
}

#[test]
fn a_disputed_order_a_closed_window_or_another_refund_to_stops_it() {
    let (m, bad) = order();
    let d = Disputed { made: &m, delivered: &bad, refund_to: REFUND_TO };
    let at = |state| [word(state), word(9), word(0), word(5), word(AT)];
    assert_eq!(d.after_read(Some(&at(3)), AT), SendStep::Done);
    assert!(matches!(d.after_read(None, AT), SendStep::Failed(_)));
    let late = AT + DISPUTE_WINDOW;
    assert_eq!(d.after_read(Some(&at(2)), late), SendStep::Failed(TOO_LATE));
    let other = d.after_refund_to([0x99; 20], (2, AT), AT);
    assert_eq!(other, SendStep::Failed(NOT_REFUND_TO));
    assert!(DISPUTE_GAS >= quote().gas_limit);
}
