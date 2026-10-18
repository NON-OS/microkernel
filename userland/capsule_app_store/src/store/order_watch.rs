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

//! The order a purchase became, as the store follows it (B9, v1.4.42): 19/3
//! TAKE_READY every TAKE_MS for its orderId, then ORDER_STATUS as a meter
//! job every WATCH_MS, for the status line. The install waits on I-C5.

use nonos_libc::mk_uptime_ms;
use nonos_payment_proto::{job_id, parse_job_id, OrderStatus, Ready, ORDER_STATUS_LEN};
use nonos_payment_proto::{EAGAIN, OP_JOB, OP_ORDER_STATUS, OP_PURCHASE, PURCHASE_TAKE_READY};

use super::asked::Asked;
use super::meter_call::call;
use super::purchase_said::ours;
use super::state::State;

const TAKE_MS: i64 = 2_000;
const WATCH_MS: i64 = 10_000;

impl State {
    /// One step of following the order. True when the status line changed.
    pub fn watch_order(&mut self) -> bool {
        let now = mk_uptime_ms();
        let o = &mut self.orders;
        if let Some(id) = o.job {
            let got = call(OP_JOB, &job_id(id), ORDER_STATUS_LEN);
            if got == Err(EAGAIN) {
                return false;
            }
            o.job = None;
            let line = got
                .ok()
                .and_then(|b| OrderStatus::parse(&b))
                .map(|s| Asked::Order(s.state, s.plain));
            let changed = line.is_some() && line != self.asked;
            self.asked = line.or(self.asked);
            return changed;
        }
        if now < o.at_ms || o.posted.is_none() {
            return false;
        }
        match o.order.as_ref() {
            None => {
                o.at_ms = now + TAKE_MS;
                let got = call(OP_PURCHASE, &[PURCHASE_TAKE_READY], 300).ok();
                let ready = got.and_then(|b| Ready::parse(&b));
                o.order = ours(ready, o.posted.as_ref());
            }
            Some(r) => {
                o.at_ms = now + WATCH_MS;
                let asked = call(OP_ORDER_STATUS, &r.order_id, 4).ok();
                o.job = asked.and_then(|b| parse_job_id(&b));
            }
        }
        false
    }
}
