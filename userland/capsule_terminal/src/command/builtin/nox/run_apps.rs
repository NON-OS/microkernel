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
//! Run a nox app or package command; returns whether it succeeded.

use super::route::apps::Apps;
use super::{apps, exec, install, pkg, run};
use crate::term::state::State;

pub(super) fn run(state: &mut State, cmd: Apps, args: &[&[u8]], rest: &[&[u8]]) -> bool {
    match cmd {
        Apps::Market => {
            apps::run(state, args);
            true
        }
        Apps::Run => run::run(state, rest),
        Apps::Exec => exec::run(state, rest),
        Apps::Install => install::run(state, rest),
        Apps::Pkg => pkg::run(state, rest),
        Apps::Git => {
            crate::command::builtin::git::run(state, rest);
            true
        }
    }
}
