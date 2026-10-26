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

//! Alt+Tab, which the input router hands only to the shell: each press
//! focuses the next window in the order they last had focus (Shift+Alt+Tab,
//! the one before), and the strip shows the run's windows for a moment.

use nonos_libc::mk_uptime_ms;

use super::escape_chord::clear_chrome;
use super::launcher_request::focus_pid;
use crate::state::ctx::switch::ORDER_MAX;
use crate::state::Context;

/// The Shift bit of a key event's modifier flags, as app_skeleton names it.
const MOD_SHIFT: u16 = nonos_app_skeleton::MOD_SHIFT;

pub fn switch(ctx: &mut Context, flags: u16) {
    let (now, back) = (mk_uptime_ms(), flags & MOD_SHIFT != 0);
    // A window whose process has gone takes no focus: it is forgotten as its
    // close would have been, and the press goes on to the next one.
    for _ in 0..ORDER_MAX {
        let Some(w) = ctx.switch.step(now, back) else { break };
        if focus_pid(w.0) {
            break;
        }
        ctx.switch.closed(w);
    }
    // Menus and the Launchpad close, and the strip is painted.
    clear_chrome(ctx);
}
