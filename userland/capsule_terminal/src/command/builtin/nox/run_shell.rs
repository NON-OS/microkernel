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
//! Run a nox shell command; every one of them succeeds.

use super::route::shell::Shell;
use super::{alias, clear, display, echo, help, history, motd, set, unalias, unset};
use crate::command::output::Output;
use crate::term::state::State;

pub(super) fn run(state: &mut State, cmd: Shell, args: &[&[u8]], rest: &[&[u8]]) -> bool {
    match cmd {
        Shell::Env => {
            set::run(state, &[]);
            true
        }
        Shell::Set => {
            set::run(state, rest);
            true
        }
        Shell::Unset => {
            unset::run(state, rest);
            true
        }
        Shell::Alias => {
            alias::run(state, rest);
            true
        }
        Shell::Unalias => {
            unalias::run(state, rest);
            true
        }
        Shell::Echo => {
            echo::run(state, args);
            true
        }
        Shell::History => {
            history::run(state, args);
            true
        }
        Shell::Display => {
            display::run(state, args);
            true
        }
        Shell::Clear => {
            clear::run(state);
            true
        }
        Shell::Help => {
            help::run(&mut Output::new(&mut state.scrollback));
            true
        }
        Shell::Motd => {
            motd::run(state);
            true
        }
    }
}
