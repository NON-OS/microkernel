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
 * A made order watched from the purchase screen (B8, the plan): its orderId
 * handed back to the store once (19/2 READY, v1.4.42), then ORDER_STATUS
 * asked every WATCH_MS as a meter job, its answer kept for the rows.
 */

use nonos_libc::mk_uptime_ms;
use nonos_payment_proto::{OrderStatus, Ready, EAGAIN, EBUSY, OP_ORDER_STATUS};

use super::intake::Step;
use crate::wallet::meter::call::start;
use crate::wallet::meter::order_calls::{order_job, ready};
use crate::wallet::state::State;

const WATCH_MS: u64 = 10_000;

#[derive(Default)]
pub struct Watch {
    pub status: Option<OrderStatus>,
    job: Option<u32>,
    at: u64,
    /// The store has the orderId, or was told all it can be.
    handed: bool,
}

/// Whether the screen changed.
pub fn poll(state: &mut State) -> bool {
    let Some(p) = state.meter.purchase.as_deref_mut() else { return false };
    let Step::Fund(m) = &p.step else { return false };
    let w = &mut p.watch;
    if !w.handed {
        let r = Ready { names: p.names.clone(), order_id: m.order_id };
        w.handed = !matches!(ready(&r), Err(EBUSY | EAGAIN));
    }
    if let Some(id) = w.job {
        return match order_job(id) {
            Err(EAGAIN) => false,
            got => {
                w.job = None;
                let status = got.ok().and_then(|b| OrderStatus::parse(&b));
                let changed = status.is_some() && status != w.status;
                w.status = status.or(w.status);
                changed
            }
        };
    }
    let now = mk_uptime_ms().max(0) as u64;
    if now >= w.at {
        w.at = now + WATCH_MS;
        w.job = start(OP_ORDER_STATUS, &m.order_id).ok();
    }
    false
}
