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

//! A line that begins with `qwen`: a window asked for, tiers to download or
//! list, help, or a chat on this terminal.

use nonos_libc::mk_time_millis;

use super::ask::parse;
use crate::command::output::Output;
use crate::jobs::submit;
use crate::term::state::State;

/// Run `line` when it is a `qwen` line. False, with nothing done, when not.
pub fn enter(state: &mut State, line: &[u8]) -> bool {
    let Some(ask) = parse(line) else { return false };
    if let Some(window) = super::window::parse(&ask) {
        super::open::open(state, ask.question, window);
        return true;
    }
    if let Some(fetch) = super::fetch_words::parse(&ask) {
        super::fetch::enter(state, fetch);
        return true;
    }
    let recorded = ask.recorded();
    state.history.push(&recorded);
    state.last_status = 0;
    if ask.tier.is_none() && matches!(ask.question, b"-h" | b"--help") {
        let _ = crate::command::builtin::help_one::run(
            &mut Output::new(&mut state.scrollback),
            b"qwen",
        );
        return true;
    }
    if let Some(work) = super::start::spawn(state, ask.tier(), ask.question) {
        let _ = submit(state, &recorded, false, work);
        state.fg_running = true;
        state.fg_started_ms = mk_time_millis();
    }
    true
}
