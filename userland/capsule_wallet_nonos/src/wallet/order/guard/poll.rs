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
 * The consent's focus guard, escrow or voucher, looked at every tick while
 * it shows (terms/arming.rs): the window manager asked who holds focus;
 * its press shown armed once held ARM_MS; focus lost over the drawn consent sends the
 * purchase back to reading its terms, nothing shown until they are read
 * again, when the consent opens afresh and the person must ask again.
 */

use nonos_libc::mk_uptime_ms;

pub use super::focus::focused;
use crate::wallet::order::intake::{Pending, Step};
use crate::wallet::order::read::Named;
use crate::wallet::order::terms::arming::{armed, look, Guard, Look};
use crate::wallet::state::{State, VIEW_PURCHASE};

/// Whether the purchase is a consent a press could act on: escrow Buy, or
/// a voucher's Open channel or Use it (the the ruling 1).
pub fn guarded(p: &Pending) -> bool {
    matches!((&p.step, &p.read), (Step::Consent, Ok(Named::Escrow(_) | Named::Voucher(_))))
}

/// Whether the consent shows and its guard is looked at each tick.
pub fn guarding(state: &State) -> bool {
    state.view == VIEW_PURCHASE && state.meter.purchase.as_deref().is_some_and(guarded)
}

pub fn now() -> u64 {
    mk_uptime_ms().max(0) as u64
}

/// One look at focus; whether the screen changed.
pub fn poll(state: &mut State) -> bool {
    let shown = guarding(state);
    let Some(p) = state.meter.purchase.as_deref_mut().filter(|p| guarded(p)) else { return false };
    let (was, now) = ((p.guard, p.armed), now());
    let seen = if shown { look(p.guard, focused(), now) } else { Look::Held(Guard::default()) };
    let Look::Held(g) = seen else {
        super::open::read_again(state);
        return true;
    };
    p.guard = g;
    p.armed = g.since.is_some_and(|since| armed(g.drawn, true, since, now));
    (p.guard, p.armed) != was
}

/// The consent was just painted whole on the window.
pub fn drawn(state: &mut State) {
    if guarding(state) {
        state.meter.purchase.iter_mut().for_each(|p| p.guard.drawn = true);
    }
}
