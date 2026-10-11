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
 * A transaction from refundTo (B8, the plan, v1.4.51): a dispute (kind 10)
 * or a refund (kind 11), each moved on by one answer: the quote (INTENT),
 * refundTo's ETH (GAS) until a Shield withdraw has paid for the gas, the
 * signature (SIGN_FROM_FRESH), the broadcast, then orderOf until it reads
 * the state the call makes. Pure: the calls are the caller's.
 */

use alloc::vec::Vec;

use nonos_settle_proto::Quote;
use shield_wire::FreshAsk;

pub const DISPUTE: u8 = 10;
pub const REFUND: u8 = 11;

/// What goes from refundTo: its kind and the calldata, unchanged.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EscrowSend {
    pub kind: u8,
    pub from: [u8; 20],
    pub calldata: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SendStep {
    Reading,
    /// orderOf's state and deliveredAt allow it: ORDER_REFUND next.
    RefundTo((u8, u64)),
    Quoting(EscrowSend),
    /// refundTo holds less than the gas can cost: a Shield withdraw of
    /// `size` wei of ETH to it, then GAS until it arrives.
    Gas { send: EscrowSend, quote: Quote, size: u128 },
    Signing(FreshAsk),
    Broadcasting(Vec<u8>),
    Sent([u8; 32]),
    /// orderOf reads Disputed, or Refunded.
    Done,
    Failed(&'static str),
}
