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
 * ORDER_DELIVERED from the wallet (B8, v1.4.46): once orderOf reads
 * Delivered (state 2) and the plaintext is not sealed yet, the meter is
 * asked with the orderId alone. It reads K through settle itself, so K
 * never passes the wallet. ETIME, no Delivered log yet, is asked again
 * after ASK_MS.
 */

use nonos_libc::mk_uptime_ms;
use nonos_payment_proto::{DeliveredMade, EAGAIN, OP_ORDER_DELIVERED};

use super::intake::Step;
use crate::wallet::meter::call::start;
use crate::wallet::meter::order_calls::order_job;
use crate::wallet::state::State;

const ASK_MS: u64 = 30_000;
const DELIVERED: u8 = 2;

#[derive(Default)]
pub struct Delivery {
    pub made: Option<DeliveredMade>,
    job: Option<u32>,
    at: u64,
}

/// Whether the screen changed.
pub fn poll(state: &mut State) -> bool {
    let Some(p) = state.meter.purchase.as_deref_mut() else { return false };
    let Step::Fund(m) = &p.step else { return false };
    let d = &mut p.delivery;
    if let Some(id) = d.job {
        return match order_job(id) {
            Err(EAGAIN) => false,
            got => {
                d.job = None;
                d.made = got.ok().and_then(|b| DeliveredMade::parse(&b)).or(d.made.take());
                d.made.is_some()
            }
        };
    }
    let due = p.watch.status.is_some_and(|s| s.state == DELIVERED && !s.plain);
    let now = mk_uptime_ms().max(0) as u64;
    if due && d.made.is_none() && now >= d.at {
        d.at = now + ASK_MS;
        d.job = start(OP_ORDER_DELIVERED, &m.order_id).ok();
    }
    false
}
