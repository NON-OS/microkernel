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
//! Run a shell command that works on files.

use super::route::files::Files;
use crate::command::builtin;
use crate::term::state::State;

pub(super) fn run(state: &mut State, cmd: Files, args: &[&[u8]]) {
    match cmd {
        Files::Pwd => builtin::fs::pwd(state),
        Files::Cd => builtin::fs::cd(state, args),
        Files::Ls => builtin::fs::ls(state, args),
        Files::Cat => builtin::fs::cat(state, args),
        Files::Mkdir => builtin::fs::mkdir(state, args),
        Files::Touch => builtin::fs::touch(state, args),
        Files::Rm => builtin::fs::rm(state, args),
        Files::Mv => builtin::fs::mv(state, args),
        Files::Cp => builtin::fs::cp(state, args),
        Files::Stat => builtin::fs::stat(state, args),
        Files::Rmdir => builtin::fs::rmdir(state, args),
        Files::Find => builtin::fs::find(state, args),
        Files::Tree => builtin::fs::tree(state, args),
        Files::Head => builtin::fs::head(state, args),
        Files::Tail => builtin::fs::tail(state, args),
        Files::Grep => builtin::fs::grep(state, args),
        Files::Wc => builtin::fs::wc(state, args),
        // The line filters that read files named after them; in a pipe the
        // same names filter the piped lines instead.
        Files::Filter => builtin::fs::filter_file(state, args),
    }
}
