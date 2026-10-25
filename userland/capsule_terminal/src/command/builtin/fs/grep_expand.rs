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

//! The files under a directory, for `grep -r` to search.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;

use super::super::{abspath, pid};
use crate::term::state::State;

pub(super) fn expand(state: &mut State, arg: &[u8], out: &mut Vec<Vec<u8>>) {
    let mut dir = abspath(state, arg);
    if dir.last() != Some(&b'/') {
        dir.push(b'/');
    }
    let owner = pid(state);
    match vfs::list_paths(owner, &dir) {
        Ok(paths) => {
            for p in paths {
                let bytes = p.into_bytes();
                if bytes.last() != Some(&b'/') {
                    out.push(bytes);
                }
            }
        }
        Err(_) => out.push(arg.to_vec()),
    }
}
