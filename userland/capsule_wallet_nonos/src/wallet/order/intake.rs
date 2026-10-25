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
 * A purchase the store named, taken from the meter's mailbox (op 19/1,
 * the plan): asked every TAKE_EVERY_MS while the wallet is open, unlocked
 * and holds no other purchase. Nothing of it shows while the market is
 * read, one call a tick; the consent opens once every answer is in and
 * the terms read (consent.rs, shown.rs).
 */

use alloc::boxed::Box;

use nonos_libc::mk_uptime_ms;

pub use super::pending::{Pending, Step};
use super::terms::shown::{unread, Unread};
use super::watch::Watch;
use crate::wallet::meter::order_calls::take;
use crate::wallet::state::State;

const TAKE_EVERY_MS: u64 = 2_000;

/// Whether a purchase arrived; its terms are read before anything shows.
pub fn take_due(state: &mut State) -> bool {
    if state.locked || state.meter.purchase.is_some() || state.meter.job.is_some() {
        return false;
    }
    let now = mk_uptime_ms().max(0) as u64;
    if now < state.meter.take_at {
        return false;
    }
    state.meter.take_at = now + TAKE_EVERY_MS;
    let Some(names) = take() else { return false };
    let Unread { read, reading, record, fees, guard } = unread();
    let (step, watch, refund_to) = (Step::Consent, Watch::default(), [0; 20]);
    let (form, delivery, held) = Default::default();
    let (escrow, armed) = (None, false);
    let p = Pending {
        names,
        read,
        reading,
        record,
        guard,
        armed,
        fees,
        watch,
        delivery,
        form,
        held,
        step,
        refund_to,
        escrow,
    };
    state.meter.purchase = Some(Box::new(p));
    true
}
