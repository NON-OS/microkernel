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
//! Route a nox name to its group and run it; set the status it leaves.

use super::route::find::{route, Nox};
use super::{help, run_apps, run_files, run_host, run_net, run_shell, unknown};
use crate::command::output::Output;
use crate::command::Outcome;
use crate::term::state::State;

pub fn dispatch(state: &mut State, args: &[&[u8]]) -> Outcome {
    if args.is_empty() {
        help::run(&mut Output::new(&mut state.scrollback));
        return Outcome::Repaint;
    }
    let rest = &args[1..];
    // Fallible commands return their success; infallible commands always
    // succeed; an unknown command is a failure. The result drives `&&`
    // and `||` sequencing through `state.last_status`.
    let ok = match route(args[0]) {
        Some(Nox::Files(cmd)) => run_files::run(state, cmd, rest),
        Some(Nox::Net(cmd)) => run_net::run(state, cmd, args, rest),
        Some(Nox::Apps(cmd)) => run_apps::run(state, cmd, args, rest),
        Some(Nox::Shell(cmd)) => run_shell::run(state, cmd, args, rest),
        Some(Nox::Host(cmd)) => run_host::run(state, cmd, args, rest),
        None => {
            unknown::run(state, args[0]);
            false
        }
    };
    state.last_status = if ok { 0 } else { 1 };
    Outcome::Repaint
}
