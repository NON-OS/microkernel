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

//! Telling the reader a page's script was stopped.

use alloc::string::String;

use nonos_qjs::Stop;

use crate::browser::omnibox::Change;
use crate::browser::state::State;

/// How long the notice stays up.
const NOTICE_MS: i64 = 8_000;

/*
 * The engine stops code that runs past its budget or its heap, and the
 * page stays as the code left it, usable. Saying nothing would leave the
 * reader with a page that silently half works, so the stop is said in the
 * status bubble at the bottom of the page and on the debug console. Run
 * once a tick, after the page's timers and whatever the tick's input and
 * fetches ran: one stop, however it came about, is reported once.
 */
/// Report a stopped script, and take the notice down when its time is up.
pub fn script_stop(state: &mut State) {
    let now = nonos_libc::mk_uptime_ms();
    if state.ui.notice.as_ref().is_some_and(|(_, until)| now >= *until) {
        state.ui.notice = None;
        state.mark(Change::Bubble);
    }
    let Some(stop) = state.engine.as_ref().and_then(|e| e.take_stop()) else {
        return;
    };
    let line = match stop {
        Stop::Time => "A script on this page ran too long and was stopped.",
        Stop::Memory => "A script on this page ran out of memory and was stopped.",
    };
    let trace = alloc::format!("[BROWSER] script stopped: {:?}\n", stop);
    nonos_libc::mk_debug(trace.as_ptr(), trace.len());
    state.status = String::from(line);
    state.ui.notice = Some((String::from(line), now + NOTICE_MS));
    state.mark(Change::Bubble);
}
