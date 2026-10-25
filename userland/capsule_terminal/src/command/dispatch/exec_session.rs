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
//! Run a shell command that works on this session.

use super::route::session::Session;
use crate::command::builtin;
use crate::command::output::Output;
use crate::term::state::State;

pub(super) fn run(state: &mut State, cmd: Session, args: &[&[u8]]) {
    match cmd {
        Session::Clear => builtin::clear::run(&mut state.scrollback, args),
        Session::Echo => builtin::echo::run(&mut Output::new(&mut state.scrollback), args),
        Session::History => {
            builtin::history_cmd::run(&mut Output::new(&mut state.scrollback), &state.history, args)
        }
        Session::Jobs => {
            builtin::jobs::run_jobs(&mut Output::new(&mut state.scrollback), &state.jobs, args)
        }
        Session::Fg => builtin::jobs::run_fg(state, args),
        Session::Bg => {
            builtin::jobs::run_bg(&mut Output::new(&mut state.scrollback), &state.jobs, args)
        }
        Session::Which => {
            let ok = builtin::which::run(&mut Output::new(&mut state.scrollback), args);
            state.last_status = i32::from(!ok);
        }
        Session::Help => {
            let mut out = Output::new(&mut state.scrollback);
            match args.get(1) {
                Some(name) => {
                    let ok = builtin::help::topic(&mut out, name)
                        || builtin::help_one::run(&mut out, name);
                    state.last_status = i32::from(!ok);
                }
                None => builtin::help::run(&mut out),
            }
        }
        Session::Theme => builtin::theme::run(state, args),
        Session::Qwen => builtin::qwen::misplaced(state),
    }
}
