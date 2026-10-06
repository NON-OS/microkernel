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
use crate::browser::fetch::{exits, fail, finish, incomplete, load_log, nav_trace, tls_reason, words};
use crate::browser::net::Source;
use crate::browser::net::mixnet::{silent_now, Network, Way};
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
    let again = |job: &mut Fetch| fail::Again {
        post: job.post.take(),
        suppress: job.suppress,
        requested: job.requested,
    };
    match (job.phase, raw) {
        /*
         * The connection closed before the response was whole. What came is
         * not the page, and showing it as one hides that it was cut; the
         * reader is told how much arrived of how much was declared.
         */
        (Phase::Decrypt | Phase::Done, Some(raw)) if job.truncated => {
            let why = incomplete::incomplete(&raw);
            trace(w, &job, &alloc::format!("response cut off: {why}"));
            fail::fail(state, &why, &why, again(&mut job));
            if state.pending_nav.is_none() {
                teardown(state, w);
            }
        }
        (Phase::Decrypt | Phase::Done, Some(raw)) => {
            /* document.cookie on this page reads the jar it was loaded from. */
            crate::browser::cookie::set_page(job.net());
            /* Where relative links, redirects and scripts resolve from now. */
            state.base = Some(job.url.clone());
            if !redirect(&raw) {
                teardown(state, w);
            }
            /* The page is built here: parsed, styled as far as its sheets
             * allow, its inline scripts run and laid out. On a slow machine
             * this is where the time goes, so the line says how long. */
            let arrived = w.now_ms();
            finish::finish(state, &raw, job.suppress);
            let built = w.now_ms().wrapping_sub(arrived);
            let what = alloc::format!(
                "{} ({}), page built in {built} ms",
                nav_trace::status_line(&raw),
                nav_trace::size(raw.len())
            );
            load_log::note(w, &nav_trace::line(&job, arrived, &what));
        }
        /*
         * A connection that was never made is reported at once, as it was
         * when the connect blocked: the connect already waited its eight
         * seconds, and two more tries would triple that before a word.
         */
        _ if job.dial.is_some() => {
            let said = tls_reason::reason(&job);
            trace(w, &job, &nav_trace::stopped(&job, &said));
            unreached(state, &said);
            teardown(state, w);
        }
        _ => {
            let mut said = tls_reason::reason(&job);
            let mut code = job.error.unwrap_or("");
            /* Did the exits go silent under it? net.socks5 says how many it
             * walked away from; more than when the navigation began means
             * this failure was theirs. */
            if let Way::Proxy { net: Network::Nym, port, .. } = job.way {
                if exits::exits_silent(Network::Nym, code, job.silent_base, silent_now(port)) {
                    code = exits::EXIT_SILENT;
                    said = words(code, job.way, &job.url.host);
                }
            }
            trace(w, &job, &nav_trace::stopped(&job, &said));
            fail::fail(state, code, &said, again(&mut job));
            if state.pending_nav.is_none() && exits::offers_anyone(code, job.net()) {
                exits::offer_anyone(state, &said);
            }
            /* No retry is coming, and the page shown is the error. */
            if state.pending_nav.is_none() {
                teardown(state, w);
            }
        }
    }
    true
}

/// The load's line on the serial console (`nav_trace`).
fn trace(w: &mut NetWire, job: &Fetch, what: &str) {
    let line = nav_trace::line(job, w.now_ms(), what);
    load_log::note(w, &line);
}
