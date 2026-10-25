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

//! The lines a `<` file feeds the command.

use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::read_file;

use crate::term::cwd::resolve;
use crate::term::state::State;

const MAX_INPUT: u32 = 65536;

pub(super) fn read_input(state: &mut State, path_arg: &[u8]) -> Vec<Vec<u8>> {
    let path = resolve(state.cwd.as_bytes(), path_arg);
    /* One byte past the most, so a longer file is refused, never fed cut. */
    match read_file(state.owner_pid, &path, MAX_INPUT + 1) {
        Ok(data) if data.len() > MAX_INPUT as usize => {
            state.scrollback.push_line(b"larger than a `<` file feeds (64 KiB)");
            Vec::new()
        }
        Ok(data) => super::super::filter::input::lines_of(&data),
        Err(e) => {
            state.scrollback.push_line(e.as_bytes());
            Vec::new()
        }
    }
}
