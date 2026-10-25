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

use nonos_app_skeleton::clients::vfs::list_paths;

use super::super::pull::target::Target;
use crate::term::state::State;

#[path = "walk_one.rs"]
mod walk_one;

use walk_one::one;

pub(super) fn walk(
    state: &mut State,
    pid: u32,
    ip: [u8; 4],
    src: &[u8],
    target: &Target,
    extra: &[u8],
) -> bool {
    let prefix = join(src, &[]);
    let entries = match list_paths(pid, &prefix) {
        Ok(e) => e,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    let mut ok = true;
    for entry in &entries {
        let local = entry.as_bytes();
        if local.ends_with(b"/") {
            continue;
        }
        let remote = join(&target.path, &local[prefix.len()..]);
        if !one(state, pid, ip, target, local, &remote, extra) {
            ok = false;
        }
    }
    ok
}

fn join(base: &[u8], name: &[u8]) -> Vec<u8> {
    let mut p = base.to_vec();
    if p.last() != Some(&b'/') {
        p.push(b'/');
    }
    p.extend_from_slice(name);
    p
}
