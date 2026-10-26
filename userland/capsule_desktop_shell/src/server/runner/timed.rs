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

//! What the serve loop wakes for besides its tick: a toast's time up, and
//! the end of a run of Alt+Tab presses, when the switcher's strip goes.

use nonos_libc::mk_uptime_ms;

use crate::state::Context;

/// The uptime of the next of the two, if either is pending.
pub(super) fn due(ctx: &Context) -> Option<i64> {
    let toast = ctx.toasts.next_expiry().map(|t| t.0);
    let strip = ctx.switch.session.as_ref().map(|s| s.until_ms());
    match (toast, strip) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (a, b) => a.or(b),
    }
}

/// Take down what is due: a toast whose time is up, the strip after a run.
pub(super) fn expire(ctx: &mut Context) {
    ctx.toasts.expire(crate::server::toast_clock::now());
    if ctx.switch.expire(mk_uptime_ms()) {
        crate::server::repaint::repaint(ctx);
    }
}
