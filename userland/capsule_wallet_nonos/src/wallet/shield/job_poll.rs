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

//! The tick's one look at the running job.

use nonos_libc::mk_uptime_ms;
use shield_wire::*;

use super::client::call;
use super::job::why;
use super::job_progress::{previewed, progress};
use super::job_steps::missed;
use crate::wallet::state::State;

/* One look at the running job. True when the screen should repaint. */
pub fn poll(state: &mut State) -> bool {
    let Some(w) = state.shield_ui.waiting else { return false };
    let elapsed = ((mk_uptime_ms() - w.since_ms).max(0) / 1000) as u32;
    let ticked = elapsed != state.shield_ui.elapsed_s;
    state.shield_ui.elapsed_s = elapsed;
    let now = mk_uptime_ms();
    if now < state.shield_ui.poll_at_ms {
        return ticked;
    }
    let a = match call(OP_RESULT, &[]) {
        Ok(a) => {
            state.shield_ui.misses = 0;
            state.shield_ui.poll_at_ms = 0;
            a
        }
        Err(_) => return missed(state, now) | ticked,
    };
    match field(&a.body, "state") {
        Some("done") => {
            state.shield_ui.waiting = None;
            super::apply::apply(state, w.op, &a.body);
            true
        }
        Some("failed") => {
            state.shield_ui.waiting = None;
            super::apply_refresh::failed(state, w.op, why(&a.body));
            true
        }
        /* The service restarted under us: nothing is running for this wallet,
         * and its store is shut. Said, and opened again by itself. */
        Some("none") => {
            state.shield_ui.waiting = None;
            state.shield_ui.opened = false;
            state.shield_ui.nox1 = None;
            super::open::restarted(state);
            true
        }
        _ => ticked | progress(state, &a.body) | previewed(state, w.op, &a.body),
    }
}
