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
//! Run a shell command that reports on the system or this terminal.

use super::route::info::Info;
use crate::command::builtin;
use crate::command::output::Output;
use crate::term::state::State;

pub(super) fn run(state: &mut State, cmd: Info, args: &[&[u8]]) {
    match cmd {
        Info::About => builtin::about::run(&mut Output::new(&mut state.scrollback), args),
        Info::Bench => builtin::bench::run(&mut Output::new(&mut state.scrollback), args),
        Info::Version => builtin::version::run(&mut Output::new(&mut state.scrollback), args),
        Info::Whoami => builtin::whoami::run(&mut Output::new(&mut state.scrollback), args),
        Info::Receipt => builtin::receipt::run(&mut Output::new(&mut state.scrollback), args),
        Info::Audit => builtin::audit::run(&mut Output::new(&mut state.scrollback), args),
        Info::Log => builtin::log::run(&mut Output::new(&mut state.scrollback), args),
        Info::Capsules => builtin::capsules::run(&mut Output::new(&mut state.scrollback), args),
        Info::Display => builtin::display::run(&mut Output::new(&mut state.scrollback), args),
        Info::Neofetch => builtin::neofetch::run(state),
        Info::Motd => builtin::motd::run(&mut state.scrollback, args),
        Info::Market => builtin::market::run(&mut Output::new(&mut state.scrollback), args),
    }
}
