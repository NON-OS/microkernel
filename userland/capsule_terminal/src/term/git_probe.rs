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

//! Reading the branch the prompt shows, once a line has run: a `cd`, a
//! `git init` or a `git clone` may each have changed it.

use nonos_app_skeleton::clients::vfs;

use super::prompt_bits::{branch, heads};
use super::state::State;

/// A HEAD file is one line; anything longer is not one.
const MAX_HEAD: u32 = 256;

pub fn probe(state: &mut State) {
    if state.owner_pid == 0 {
        state.owner_pid = nonos_libc::mk_getpid();
    }
    let owner = state.owner_pid;
    state.git_branch.clear();
    for path in heads(state.cwd.as_bytes()) {
        if let Ok(head) = vfs::read_file(owner, &path, MAX_HEAD) {
            if let Some(name) = branch(&head) {
                state.git_branch.extend_from_slice(name);
            }
            return;
        }
    }
}
