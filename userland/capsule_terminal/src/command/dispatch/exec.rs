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
//! Route a shell name to its group and run it; unknown names go to nox.

use super::outcome::Outcome;
use super::route::find::{route, Shell};
use super::{exec_files, exec_info, exec_net, exec_session};
use crate::command::builtin;
use crate::term::state::State;

/// Run one resolved command; the caller strips redirects and detects exit.
pub(super) fn exec(state: &mut State, args: &[&[u8]]) -> Outcome {
    if args.is_empty() {
        return Outcome::Repaint;
    }
    match route(args[0]) {
        Some(Shell::Nox) => return builtin::nox::dispatch(state, &args[1..]),
        Some(Shell::Info(cmd)) => exec_info::run(state, cmd, args),
        Some(Shell::Session(cmd)) => exec_session::run(state, cmd, args),
        Some(Shell::Files(cmd)) => exec_files::run(state, cmd, args),
        Some(Shell::Net(cmd)) => exec_net::run(state, cmd, args),
        None => return builtin::nox::dispatch(state, args),
    }
    Outcome::Repaint
}
