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
use nonos_app_skeleton::clients::vfs::read_file;

use super::exec::exec;
use super::outcome::Outcome;
use super::pipeline::{run_filters, run_pipeline};
use super::redirect::{plan, Sink, Source};
use super::write_redirect::write_redirect;
use crate::command::builtin;
use crate::command::parse::Argv;
use crate::term::cwd::resolve;
use crate::term::state::State;

const MAX_INPUT: u32 = 65536;

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
    match plan.output {
        Sink::File { path, append } => write_redirect(state, &lines, append, path),
        Sink::Null => {}
        Sink::Screen => {
            for line in &lines {
                state.scrollback.push_line(line);
            }
        }
    }
    Outcome::Repaint
}
fn read_input(state: &mut State, path_arg: &[u8]) -> Vec<Vec<u8>> {
    let path = resolve(state.cwd.as_bytes(), path_arg);
    match read_file(state.owner_pid, &path, MAX_INPUT) {
        Ok(data) => super::filter::input::lines_of(&data),
        Err(e) => {
            state.scrollback.push_line(e.as_bytes());
            Vec::new()
        }
    }
}
