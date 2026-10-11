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
 * A refund from refundTo (B8, kind 11, the plan): refund(T, refundTo,
 * refundSalt), built here from the order's own terms and ORDER_REFUND's
 * answer. It pays only the committed refundTo, so the checks are when:
 * a Disputed order; at or after deliveryDeadline, a Funded order or NOX
 * waiting at the deposit address of one not funded; or extra above the
 * price after Released or Refunded. Sent from refundTo and no other address.
 */

use nonos_payment_proto::OrderMade;

use super::dispute_said::NOT_REFUND_TO;
use super::order_of::tail_u64;
use super::refund_call::refund_call;
use super::refund_said::*;
use super::send_step::{EscrowSend, REFUND};

/// The terms' deliveryDeadline word.
const DEADLINE_WORD: usize = 6;

/// The order's own deliveryDeadline, in unix seconds.
pub fn deadline(m: &OrderMade) -> Option<u64> {
    tail_u64(&m.terms[DEADLINE_WORD])
}

/// The refund to send, or why not. `answered` is ORDER_REFUND's refundTo
/// and refundSalt; `pending` the NOX waiting at the deposit address.
pub fn refund(
    m: &OrderMade,
    drawn: [u8; 20],
    answered: ([u8; 20], [u8; 32]),
    of: (u8, u128),
    pending: u128,
    now: u64,
) -> Result<EscrowSend, &'static str> {
    let (refund_to, salt) = answered;
    if drawn == [0; 20] || refund_to != drawn {
        return Err(NOT_REFUND_TO);
    }
    let deadline = deadline(m).ok_or(NOTHING_LEFT)?;
    match of {
        (3, _) => {}
        (4 | 5, extra) if extra > 0 => {}
        (5, _) => return Err(ALREADY),
        (2, _) => return Err(IN_WINDOW),
        (1, _) if now < deadline => return Err(NOT_YET),
        (0, _) if pending > 0 && now < deadline => return Err(NOT_YET),
        (1, _) => {}
        (0, _) if pending > 0 => {}
        _ => return Err(NOTHING_LEFT),
    }
    let calldata = refund_call(m, &refund_to, &salt).ok_or(NOTHING_LEFT)?;
    Ok(EscrowSend { kind: REFUND, from: refund_to, calldata })
}
