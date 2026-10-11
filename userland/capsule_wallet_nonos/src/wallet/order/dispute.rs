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
 * A bad chunk disputed (B8, kind 10, plan/22, plan/26): dispute()'s
 * calldata is the meter's, from ORDER_DELIVERED's BadChunk, sent as it
 * came. The checks here only refuse: the calldata must be dispute() over
 * this order's own terms, it goes from refundTo and no other address, and
 * only while orderOf reads Delivered with DELIVER_MARGIN left of the window
 * (deliveredAt + the terms' disputeWindow).
 */

use alloc::vec::Vec;

use nonos_nox_config::DELIVER_MARGIN;
use nonos_payment_proto::{DeliveredMade, OrderMade};

use super::dispute_said::*;
use super::order_of::{window_end, DELIVERED};
use super::send_step::{EscrowSend, DISPUTE};


fn selector() -> Option<[u8; 4]> {
    let k = nonos_settle_proto::KINDS.iter().find(|k| k.id == DISPUTE)?;
    Some(k.calls[0].1.to_be_bytes())
}

/// The dispute to send from `from`, or why not.
pub fn dispute(
    m: &OrderMade,
    made: &DeliveredMade,
    refund_to: [u8; 20],
    from: [u8; 20],
    of: (u8, u64),
    now: u64,
) -> Result<EscrowSend, &'static str> {
    let DeliveredMade::BadChunk { calldata, .. } = made else { return Err(NOTHING) };
    if refund_to == [0; 20] || from != refund_to {
        return Err(NOT_REFUND_TO);
    }
    let terms: Vec<u8> = m.terms.concat();
    let ours = calldata.get(..4) == selector().as_ref().map(|s| &s[..])
        && calldata.get(4..4 + terms.len()) == Some(&terms[..]);
    if !ours {
        return Err(NOT_THIS_ORDER);
    }
    let end = window_end(m, of.1).filter(|_| of.0 == DELIVERED && of.1 > 0).ok_or(NOT_DELIVERED)?;
    if end.saturating_sub(now) < DELIVER_MARGIN {
        return Err(TOO_LATE);
    }
    Ok(EscrowSend { kind: DISPUTE, from, calldata: calldata.clone() })
}
