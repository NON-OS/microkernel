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
 * A dispute or a refund from refundTo, one step a tick (B8, v1.4.51 to
 * v1.4.53): orderOf, refundTo, the quote and the gas through settle ASYNC,
 * the signature from the Shield, the broadcast, then orderOf until it
 * lands. The window never waits: settle's answers arrive by peer_frame.
 */

use nonos_libc::mk_uptime_ms;

use super::escrow_broadcast::broadcast;
use super::escrow_read::read;
use super::escrow_refund::refund_to;
use super::escrow_sign::sign;
use super::escrow_tx::{gas, quoting};
use super::escrowing::Ctx;
use super::intake::Step;
use super::read::Named;
use super::send_step::SendStep;
use crate::wallet::state::State;

/// Whether the screen changed.
pub fn poll(state: &mut State) -> bool {
    let now_ms = mk_uptime_ms().max(0) as u64;
    let timed = state.meter.settle.tick(now_ms);
    let signing = state.meter.purchase.as_deref().and_then(|p| p.escrow.as_ref());
    if signing.is_some_and(|e| matches!(e.step, SendStep::Signing(_))) {
        return sign(state) | timed;
    }
    let settle = &mut state.meter.settle;
    let Some(p) = state.meter.purchase.as_deref_mut() else { return timed };
    let (Step::Fund(m), Ok(Named::Escrow(b))) = (&p.step, &p.read) else { return timed };
    let Some(e) = p.escrow.as_mut() else { return timed };
    let c = Ctx {
        chain_id: b.chain_id,
        made: m,
        delivered: p.delivery.made.as_ref(),
        refund_to: p.refund_to,
        pending: p.watch.status.as_ref().map_or(0, |s| s.pending),
        now_ms,
        now_s: nonos_tls::rtc_now(),
    };
    let moved = match &e.step {
        SendStep::Reading | SendStep::Sent(_) => read(settle, e, &c),
        SendStep::RefundTo(of) => refund_to(e, &c, *of),
        SendStep::Quoting(_) => quoting(settle, e, &c),
        SendStep::Gas { .. } => gas(settle, e, &c),
        SendStep::Broadcasting(_) => broadcast(settle, e, &c),
        _ => false,
    };
    moved | timed
}
