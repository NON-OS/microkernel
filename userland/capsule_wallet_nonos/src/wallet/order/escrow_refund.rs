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

//! ORDER_REFUND for a dispute or a refund (v1.4.51): the order's refundTo,
//! which must be the one drawn at Buy, and for a refund its refundSalt.

use super::dispute_steps::Disputed;
use super::escrow_read::NO_ORDER;
use super::escrowing::{Ctx, Escrowing};
use super::refund::refund;
use super::send_step::SendStep;
use crate::wallet::meter::order_calls::order_refund;

const NO_REFUND_TO: &str = "The meter did not name the order's refund address.";

/// A refund straight from orderOf: ORDER_REFUND, then the checks.
pub(super) fn refund_now(c: &Ctx, of: Option<(u8, u128)>) -> SendStep {
    let Some(of) = of else { return SendStep::Failed(NO_ORDER) };
    let Some(answered) = order_refund(&c.made.order_id) else {
        return SendStep::Failed(NO_REFUND_TO);
    };
    match refund(c.made, c.refund_to, answered, of, c.pending, c.now_s) {
        Ok(send) => SendStep::Quoting(send),
        Err(why) => SendStep::Failed(why),
    }
}

/// A dispute's refundTo from ORDER_REFUND, which must be the one drawn at Buy.
pub fn refund_to(e: &mut Escrowing, c: &Ctx, of: (u8, u64)) -> bool {
    e.step = match (order_refund(&c.made.order_id), c.delivered) {
        (Some((to, _)), Some(delivered)) => {
            let d = Disputed { made: c.made, delivered, refund_to: c.refund_to };
            d.after_refund_to(to, of, c.now_s)
        }
        _ => SendStep::Failed(NO_REFUND_TO),
    };
    true
}
