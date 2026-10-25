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
use crate::term::dimensions::LINE_MAX;
use crate::term::state::State;

use super::echo_line::echo_line;
use super::run_line::run_line;

#[path = "enter_expand.rs"]
mod enter_expand;
use enter_expand::expand_line;

pub fn on_enter(state: &mut State) -> EventOutcome {
    // Running a line ends any search that found it. The match is already on
    // the line, so accepting it is simply leaving the mode. A search whose
    // needle matches nothing runs nothing: the line shows an older match.
    if !super::search::search_enter(state) {
        return EventOutcome::Repaint;
    }
    state.fresh = false;
    let started = mk_time_millis();
    state.open_block(crate::term::rtc::rtc_hms());
    let mut entered = [0u8; LINE_MAX];
    let Some(n) = expand_line(state, &mut entered) else {
        return EventOutcome::Repaint;
    };
    echo_line(state, &entered[..n]);
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
