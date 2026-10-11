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

//! What a made order offers beside Close (v1.4.51): a dispute only for a
//! bad chunk on a Delivered order, a refund only once the escrow allows it,
//! the gas while a send waits for it, and a send again after a failure.

use nonos_payment_proto::{DeliveredMade, OrderStatus};
use nonos_settle_proto::Quote;

use super::fixture::REFUND_TO;
use crate::order::escrow_offer::{offer, Offer};
use crate::order::send_step::{EscrowSend, SendStep};

const DEADLINE: u64 = 1_900_000_000;

fn status(state: u8, pending: u128) -> OrderStatus {
    OrderStatus { state, funded: 9, pending, extra: 0, plain: false }
}

#[test]
fn each_state_offers_its_action_or_none() {
    let bad = DeliveredMade::BadChunk { index: 1, calldata: vec![] };
    let ok = DeliveredMade::Ok { content_len: 1 };
    let at = |s, d, now| offer(None, Some(&s), d, DEADLINE, now);
    assert_eq!(at(status(2, 0), Some(&bad), 0), Some(Offer::Dispute));
    assert_eq!(at(status(2, 0), Some(&ok), 0), None, "a good delivery");
    assert_eq!(at(status(2, 0), None, 0), None, "not opened yet");
    assert_eq!(at(status(3, 0), None, 0), Some(Offer::Refund), "disputed: at once");
    assert_eq!(at(status(1, 0), None, DEADLINE - 1), None, "funded, before the deadline");
    assert_eq!(at(status(1, 0), None, DEADLINE), Some(Offer::Refund));
    assert_eq!(at(status(0, 5), None, DEADLINE), Some(Offer::Refund), "NOX waiting");
    assert_eq!(at(status(0, 0), None, DEADLINE), None, "nothing at the deposit");
    assert_eq!(at(status(4, 0), None, DEADLINE), None, "released, nothing waiting");
    assert_eq!(at(status(5, 0), None, DEADLINE), None, "refunded, nothing waiting");
    assert_eq!(at(status(4, 1), None, 0), Some(Offer::Refund), "released, NOX waiting");
    assert_eq!(at(status(5, 1), None, 0), Some(Offer::Refund), "refunded, NOX waiting");
    let extra = |state| OrderStatus { extra: 3, ..status(state, 0) };
    assert_eq!(at(extra(4), None, 0), Some(Offer::Refund), "released, extra alone");
    assert_eq!(at(extra(5), None, 0), Some(Offer::Refund), "refunded, extra alone");
    let most = OrderStatus { extra: u128::MAX, ..status(4, u128::MAX) };
    assert_eq!(at(most, None, 0), Some(Offer::Refund), "no overflow");
    assert_eq!(offer(None, None, Some(&bad), DEADLINE, DEADLINE), None, "no status yet");
}

#[test]
fn a_send_under_way_offers_only_its_gas_or_a_retry() {
    let send = EscrowSend { kind: 10, from: REFUND_TO, calldata: vec![] };
    let quote =
        Quote { kind: 10, to: [0; 20], nonce: 0, gas_limit: 1, max_fee: 1, max_priority: 0 };
    let gas = SendStep::Gas { send: send.clone(), quote, size: 1 };
    let s = status(3, 0);
    assert_eq!(offer(Some(&gas), Some(&s), None, 0, 0), Some(Offer::PayGas));
    assert_eq!(offer(Some(&SendStep::Failed("x")), Some(&s), None, 0, 0), Some(Offer::Again));
    assert_eq!(offer(Some(&SendStep::Quoting(send)), Some(&s), None, 0, 0), None);
    assert_eq!(offer(Some(&SendStep::Sent([0; 32])), Some(&s), None, 0, 0), None);
    assert_eq!(
        offer(Some(&SendStep::Done), Some(&s), None, 0, 0),
        Some(Offer::Refund),
        "after Disputed"
    );
}
