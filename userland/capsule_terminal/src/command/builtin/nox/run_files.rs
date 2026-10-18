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
//! Run a nox file command; returns whether it succeeded.

use super::route::files::Files;
use super::{
    copy, du, enter, find, keep, ls, mk, mv, pathname, pull, push, read, rm, stat, touch, whereis,
    write,
};
use crate::term::state::State;

/// The files group reads only the arguments after the name.
pub(super) fn run(state: &mut State, cmd: Files, rest: &[&[u8]]) -> bool {
    match cmd {
        Files::Where => {
            whereis::run(state);
            true
        }
        Files::In => enter::run(state, rest),
        Files::Ls => {
            ls::run(state, rest);
            true
        }
        Files::Read => read::run(state, rest),
        Files::Pull => pull::run(state, rest),
        Files::Push => push::run(state, rest),
        Files::Write => write::run(state, rest),
        Files::Copy => copy::run(state, rest),
        Files::Mk => mk::run(state, rest),
        Files::Rm => rm::run(state, rest),
        Files::Keep => keep::run(state, rest),
        Files::Mv => mv::run(state, rest),
        Files::Stat => stat::run(state, rest),
        Files::Find => find::run(state, rest),
        Files::Du => du::run(state, rest),
        Files::Touch => touch::run(state, rest),
        Files::Basename => pathname::basename(state, rest),
        Files::Dirname => pathname::dirname(state, rest),
    }
}
