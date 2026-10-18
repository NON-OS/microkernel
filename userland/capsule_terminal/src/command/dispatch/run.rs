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

use alloc::vec::Vec;

use super::exec::exec;
use super::outcome::Outcome;
use super::pipeline::{run_filters, run_pipeline};
use super::redirect::{plan, Sink, Source};
use crate::command::builtin;
use crate::command::parse::Argv;
use crate::term::state::State;

#[path = "run_sink.rs"]
mod run_sink;
#[path = "run_source.rs"]
mod run_source;

use run_sink::deliver;
use run_source::read_input;

pub fn run(state: &mut State, argv: &Argv<'_>) -> Outcome {
    if argv.argc == 0 {
        return Outcome::Repaint;
    }
    let args = &argv.argv[..argv.argc];
    if builtin::exit_check::want_exit(args) {
        return Outcome::Exit;
    }
    let plan = match plan(args) {
        Ok(p) => p,
        Err(msg) => {
            state.scrollback.push_error(msg);
            state.last_status = 1;
            return Outcome::Repaint;
        }
    };
    let cmd = &plan.words[..];
    let piped = cmd.iter().any(|a| *a == b"|");
    if plan.input == Source::Terminal && !piped && plan.output == Sink::Screen {
        return exec(state, cmd);
    }
    let lines = match plan.input {
        Source::File(p) => {
            let seed = read_input(state, p);
            run_filters(seed, cmd)
        }
        /* Nothing to read: the filters run over no lines. */
        Source::Null => run_filters(Vec::new(), cmd),
        Source::Terminal if piped => run_pipeline(state, cmd),
        Source::Terminal => {
            state.scrollback.begin_capture();
            let _ = exec(state, cmd);
            state.scrollback.end_capture()
        }
    };
    deliver(state, plan.output, lines);
    Outcome::Repaint
}
