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

//! renameat2's checks: its flags, both names, and the tree they are in.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::at::resolve_at;
use super::super::meta::look;
use super::super::resolve::key;
use super::super::synth;
use super::store::move_name;

const RENAME_NOREPLACE: u64 = 1;

pub fn renameat2(guest: &Guest, olddir: u64, old: u64, newdir: u64, new: u64, flags: u64) -> u64 {
    if flags & !RENAME_NOREPLACE != 0 {
        return errno::fail(errno::EINVAL);
    }
    let from = match resolve_at(guest, olddir, old) {
        Ok(from) => from,
        Err(e) => return errno::fail(e),
    };
    let to = match resolve_at(guest, newdir, new) {
        Ok(to) => to,
        Err(e) => return errno::fail(e),
    };
    let there = look(&to).is_some() || guest.links.target(&to).is_some();
    if flags & RENAME_NOREPLACE != 0 && there {
        return errno::fail(errno::EEXIST);
    }
    if [&from, &to].iter().any(|p| synth::owns(p) || key(p).writable().is_err()) {
        return errno::fail(errno::EROFS);
    }
    if from == to {
        return errno::ok(0);
    }
    if guest.links.rename(&from, to.clone()) {
        return errno::ok(0);
    }
    move_name(&from, &to)
}
