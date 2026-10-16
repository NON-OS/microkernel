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
 * A purchase's steps after consent (B8): ORDER_OFFER in the meter (started
 * by Buy, screen/purchase), then ORDER_NEW on the offer it checked, each a
 * job asked on every tick, then the deposit address to fund. The window
 * never waits on the network.
 */

use nonos_libc::mk_time_millis;
use nonos_payment_proto::OP_ORDER_NEW;

use super::deposit_check::pinned;
use super::intake::Step;
use super::purchase::{after_offer, after_order, After};
use super::read::Named;
use super::said::{failed, Stage};
use crate::wallet::meter::call::start;
use crate::wallet::meter::order_calls::order_job;
use crate::wallet::publish::Kec;
use crate::wallet::state::State;

/// Whether a running step moved on.
pub fn poll(state: &mut State) -> bool {
    let Some(p) = state.meter.purchase.as_mut() else { return false };
    let Ok(Named::Escrow(buy)) = &p.read else { return false };
    let next = match &p.step {
        Step::Offering(id) => {
            let now = mk_time_millis().max(0) as u64 / 1000;
            after_offer(buy, order_job(*id).as_deref().map_err(|e| *e), p.refund_to, now)
        }
        Step::Ordering(id, sent) => {
            let r = order_job(*id);
            after_order(&Kec, sent, pinned(sent.chain_id), r.as_deref().map_err(|e| *e))
        }
        _ => return false,
    };
    p.step = match next {
        After::Wait => return false,
        After::Order(n) => match start(OP_ORDER_NEW, &n.encode()) {
            Ok(id) => Step::Ordering(id, n),
            Err(s) => Step::Failed(failed(Stage::Order, s)),
        },
        After::Fund(m) => Step::Fund(m),
        After::Failed(why) => Step::Failed(why),
    };
    true
}
