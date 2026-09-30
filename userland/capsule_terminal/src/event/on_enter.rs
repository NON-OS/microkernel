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

use nonos_app_skeleton::EventOutcome;
use nonos_libc::mk_time_millis;

use crate::command;
use crate::term::context::context_line;
use crate::term::cwd::home_var;
use crate::term::dimensions::LINE_MAX;
use crate::term::identity::{hostname, USER};
use crate::term::prompt::PROMPT_BYTES;
use crate::term::state::State;
use crate::term::util::{copy_into, format_u64};

use super::run_line::run_line;

pub fn on_enter(state: &mut State) -> EventOutcome {
    // Running a line ends any search that found it. The match is already on
    // the line, so accepting it is simply leaving the mode.
    super::search::search_accept(state);
    state.fresh = false;
    let started = mk_time_millis();
    state.open_block(crate::term::rtc::rtc_hms());
    let mut ctx = [0u8; LINE_MAX];
    let cn = context_line(USER, hostname(), state.cwd.as_bytes(), home_var(state), &mut ctx);
    state.scrollback.push_line(&ctx[..cn]);
    // A `!` form is resolved before anything else sees the line, so what is
    // echoed, recorded in history and run are all the same text. Expanding
    // later would put one command on screen and another through the parser.
    let mut entered = [0u8; LINE_MAX];
    let n;
    /*
     * A line that starts with `qwen` is a question, and a `!word` in it is
     * part of what is asked, not a history reference.
     */
    let expanded = if command::builtin::qwen::is_line(state.line.as_bytes()) {
        None
    } else {
        crate::term::history::expand(state.line.as_bytes(), &state.history)
    };
    match expanded {
        // The expansion is what gets echoed, which is the whole safety of the
        // feature: the reader sees the command that is about to run, not the
        // shorthand they typed for it.
        Some(Ok(line)) => {
            n = line.len().min(LINE_MAX);
            entered[..n].copy_from_slice(&line[..n]);
        }
        Some(Err(_)) => {
            // Naming an entry that is not there runs nothing. Silently
            // dropping the `!` would run the rest of the line, which is how
            // history expansion earns its reputation.
            state.scrollback.push_line(b"no matching history entry");
            state.line.clear();
            state.history.reset_cursor();
            state.last_status = 1;
            state.scrollback.jump_bottom();
            return EventOutcome::Repaint;
        }
        None => {
            let body = state.line.as_bytes();
            n = body.len();
            entered[..n].copy_from_slice(body);
        }
    }
    let mut echo = [0u8; LINE_MAX + 8];
    let mut k = 0;
    k += copy_into(&mut echo[k..], PROMPT_BYTES);
    k += copy_into(&mut echo[k..], &entered[..n]);
    state.scrollback.push_line(&echo[..k]);
    /*
     * A question to Qwen is the rest of the line as typed. It is taken
     * before the shell splits and expands the line, which would break it at
     * a `;` or an apostrophe, and it is sent to the program, not kept in
     * history.
     */
    let outcome = if command::builtin::qwen::enter(state, &entered[..n]) {
        command::Outcome::Repaint
    } else {
        state.history.push(&entered[..n]);
        run_line(state, &entered[..n])
    };
    if !state.fg_running {
        let dur = (mk_time_millis() - started).clamp(0, u32::MAX as i64) as u32;
        state.close_block(state.last_status == 0, dur);
    }
    state.evict_blocks();
    state.line.clear();
    state.scrollback.jump_bottom();
    match outcome {
        command::Outcome::Exit => EventOutcome::Close,
        command::Outcome::Repaint => EventOutcome::Repaint,
    }
}

// "[n] started" line printed when a background job is submitted; the
// job's own output streams into the scrollback as Task 13's on_tick pump
// steps it.
pub(super) fn print_started(state: &mut State, id: u32) {
    let mut num = [0u8; 20];
    let nk = format_u64(id as u64, &mut num);
    let mut msg = [0u8; 32];
    let mut mk = 0;
    msg[mk] = b'[';
    mk += 1;
    mk += copy_into(&mut msg[mk..], &num[..nk]);
    mk += copy_into(&mut msg[mk..], b"] started");
    state.scrollback.push_line(&msg[..mk]);
}
