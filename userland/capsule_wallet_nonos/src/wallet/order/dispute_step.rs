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

/*
 * A dispute's steps from the wallet (B8, kind 10, v1.4.51), each moved on
 * by one answer: orderOf (settle READ 20), refundTo (ORDER_REFUND), the
 * quote (INTENT), refundTo's ETH (GAS) until a Shield withdraw has paid
 * for the gas, the signature (SIGN_FROM_FRESH), the broadcast, then
 * orderOf until it reads Disputed. Pure: the calls are the caller's.
 */

use nonos_payment_proto::{DeliveredMade, OrderMade};
use nonos_settle_proto::Quote;
use shield_wire::FreshAsk;

use super::dispute::DisputeSend;

pub const DISPUTED: u8 = 3;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DisputeStep {
    Reading,
    /// orderOf's state and deliveredAt, the window open: ORDER_REFUND next.
    RefundTo((u8, u64)),
    Quoting(DisputeSend),
    /// refundTo holds less than the gas can cost: a Shield withdraw of
    /// `size` wei of ETH to it, then GAS until it arrives.
    Gas {
        send: DisputeSend,
        quote: Quote,
        size: u128,
    },
    Signing(FreshAsk),
    Broadcasting(alloc::vec::Vec<u8>),
    Sent([u8; 32]),
    Disputed,
    Failed(&'static str),
}

/// The order, its bad chunk, and refundTo drawn at Buy.
pub struct Disputed<'a> {
    pub made: &'a OrderMade,
    pub delivered: &'a DeliveredMade,
    pub refund_to: [u8; 20],
    pub chain_id: u64,
}
