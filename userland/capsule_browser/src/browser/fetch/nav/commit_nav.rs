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

//! A finished navigation: commit its page, or say why there is none.

use super::outcome::{redirect, unreached};
use super::teardown::teardown;
use crate::browser::fetch::land::free;
use crate::browser::fetch::land::respond::{dead_kept, response};
use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::fetch::{fail, finish, tls_reason};
use crate::browser::state::State;

/// Land the navigation `job`. Always a change worth drawing.
pub(in crate::browser::fetch) fn land_nav(
    state: &mut State,
    w: &mut NetWire,
    mut job: Fetch,
) -> bool {
    if dead_kept(&job) && super::relaunch_nav::relaunch_nav(state, w, &mut job) {
        return true;
    }
    let raw = response(&mut job);
    free(state, w, &mut job, raw.as_deref());
    match (job.phase, raw) {
        (Phase::Decrypt | Phase::Done, Some(raw)) => {
            /* Where relative links, redirects and scripts resolve from now. */
            state.base = Some(job.url.clone());
            if !redirect(&raw) {
                teardown(state, w);
            }
            finish::finish(state, &raw, job.suppress);
        }
        /*
         * A connection that was never made is reported at once, as it was
         * when the connect blocked: the connect already waited its eight
         * seconds, and two more tries would triple that before a word.
         */
        _ if job.dial.is_some() => {
            unreached(state, &tls_reason::reason(&job));
            teardown(state, w);
        }
        _ => {
            let reason = tls_reason::reason(&job);
            fail::fail(state, &reason);
            /* No retry is coming, and the page shown is the error. */
            if state.pending_nav.is_none() {
                teardown(state, w);
            }
        }
    }
    true
}
