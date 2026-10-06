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

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::stat_full;

use super::listing_state::read_until_silent;
use super::state::State;

const META_STAT_LIMIT: usize = 128;

pub fn fill_meta(state: &mut State) {
    if state.all.len() > META_STAT_LIMIT {
        return;
    }
    let pid = state.owner_pid;
    // One stat a file, each waiting out the reply timeout when the store does
    // not answer: the first silence leaves the rest without sizes.
    let files: Vec<usize> = (0..state.all.len()).filter(|&i| !state.all[i].is_dir).collect();
    let all = &mut state.all;
    let read = |i: usize| stat_full(pid, all[i].full_path.as_bytes());
    let mut got: Vec<(usize, (u64, bool, u64, bool))> = Vec::new();
    read_until_silent(&files, read, |i, stat| got.push((i, stat)));
    for (i, (size, is_dir, mtime, writable)) in got {
        if !is_dir {
            all[i].size = Some(size);
            all[i].mtime = mtime;
            all[i].writable = writable;
        }
    }
}
