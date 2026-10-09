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

use nonos_libc::mk_time_millis;

use crate::command;
use crate::jobs;
use crate::term::state::State;

use super::on_enter::print_started;

/// Run each statement of `line` in turn, gated by `&&` and `||`, until one
/// starts a foreground job or asks the terminal to exit.
pub(super) fn run_line(state: &mut State, line: &[u8]) -> command::Outcome {
    let mut outcome = command::Outcome::Repaint;
    let mut prev_status: i32 = state.last_status;
    for command::Stmt { conn, body, background } in command::split_program(line) {
        let go = match conn {
            command::Conn::Always => true,
            command::Conn::And => prev_status == 0,
            command::Conn::Or => prev_status != 0,
        };
        if !go {
            continue;
        }
        let aliased = command::alias_expand(body, &state.aliases);
        let expanded = command::expand(&aliased, &state.vars, prev_status);
        state.last_status = 0;
        let argv = command::parse(&expanded);
        let args = &argv.argv[..argv.argc];
        match jobs::is_job_command(state, args) {
            jobs::Verdict::Job(work) => {
                let id = jobs::submit(state, body, background, work);
                if background {
                    print_started(state, id);
                    prev_status = state.last_status;
                    continue;
                }
                state.fg_running = true;
                state.fg_started_ms = mk_time_millis();
                break;
            }
            jobs::Verdict::Handled => {
                prev_status = state.last_status;
                continue;
            }
            jobs::Verdict::Instant => {}
        }
        if let command::Outcome::Exit = command::run(state, &argv) {
            outcome = command::Outcome::Exit;
            break;
        }
        if state.fg_running {
            break;
        }
        prev_status = state.last_status;
    }
    outcome
}
