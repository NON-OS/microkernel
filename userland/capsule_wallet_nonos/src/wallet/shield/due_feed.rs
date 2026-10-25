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

//! What fills the due list (B7, v1.4.56), at the moment money becomes due:
//! a refund or dispute this wallet sent from refundTo, and an order whose
//! escrow owes a refund there. Nothing else adds an address.

use nonos_libc::mk_uptime_ms;

use super::due::Why;
use super::due_why::due_why;
use crate::wallet::order::intake::Step;
use crate::wallet::order::refund::deadline;
use crate::wallet::state::State;

/// Why the purchase makes its refundTo due now, if it does.
fn why_now(state: &State, now_s: u64) -> Option<Why> {
    let p = state.meter.purchase.as_deref()?;
    let Step::Fund(m) = &p.step else { return None };
    let send = p.escrow.as_ref().map(|e| (e.kind, &e.step));
    let due = deadline(m).unwrap_or(u64::MAX);
    due_why(send, p.watch.status.as_ref(), p.delivery.made.as_ref(), due, now_s)
}

pub fn feed(state: &mut State) {
    let Some(why) = why_now(state, crate::wallet::clock::now_s()) else { return };
    let Some(to) = state.meter.purchase.as_deref().map(|p| p.refund_to) else { return };
    if to != [0; 20] {
        state.meter.landed.due.add(to, why, mk_uptime_ms().max(0) as u64);
    }
}
