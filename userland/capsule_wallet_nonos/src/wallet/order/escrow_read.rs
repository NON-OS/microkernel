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

//! A dispute's or a refund's reads (v1.4.53): orderOf through settle's READ
//! 20, ASYNC, and ORDER_REFUND from the meter; after the broadcast, orderOf
//! every READ_EVERY_MS until it reads the state the call makes.

use nonos_settle_proto::{parse_read_reply, Read, OP_READ};

use super::dispute_steps::{landed, Disputed};
use super::escrow_refund::refund_now;
use super::escrowing::{Ctx, Escrowing};
use super::order_of::window_end;
use super::refund_call::{refunded, state_and_extra};
use super::send_step::{SendStep, DISPUTE};
use crate::wallet::settle::{SettleCall, ESCROW};

const READ_EVERY_MS: u64 = 10_000;
const ORDER_OF: u16 = 20;
pub(super) const NO_ORDER: &str = "settle did not read the escrow's order.";

/// Ask orderOf when due; take its answer. Whether the step moved.
pub fn read(settle: &mut SettleCall, e: &mut Escrowing, c: &Ctx) -> bool {
    let Some(r) = settle.take(OP_READ, ESCROW) else {
        if c.now_ms >= e.next_ms {
            let body =
                Read { chain_id: c.chain_id, view: ORDER_OF, args: [c.made.order_id].into() };
            let asked = body.encode().map(|b| settle.ask(OP_READ, ESCROW, &b, c.now_ms));
            e.next_ms = c.now_ms + READ_EVERY_MS;
            return matches!(asked, Some(Ok(())));
        }
        return false;
    };
    let words = r.ok().and_then(|b| parse_read_reply(&b));
    let w = words.as_deref();
    e.step = match (&e.step, e.kind, c.delivered) {
        (SendStep::Sent(_), DISPUTE, _) if landed(w) => SendStep::Done,
        (SendStep::Sent(_), _, _) if e.kind != DISPUTE && refunded(w) => SendStep::Done,
        (SendStep::Sent(h), _, _) => SendStep::Sent(*h),
        (_, DISPUTE, Some(delivered)) => {
            let d = Disputed { made: c.made, delivered, refund_to: c.refund_to };
            d.after_read(w, c.now_s)
        }
        (_, DISPUTE, None) => SendStep::Failed(super::dispute_said::NOTHING),
        _ => refund_now(c, w.and_then(state_and_extra)),
    };
    if let SendStep::RefundTo((_, at)) = e.step {
        e.window_end = window_end(c.made, at);
    }
    true
}
