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

/* The xattr calls by path and by descriptor. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::super::{at, flags::AT_FDCWD, meta, walk};
use super::answer::answer;

pub enum Op {
    Get,
    Set,
    List,
    Remove,
}

/* The call's own arguments after the path or descriptor. */
pub struct Args {
    pub name: u64,
    pub value: u64,
    pub size: u64,
    pub flags: u64,
}

pub fn by_path(guest: &Guest, path: u64, op: Op, args: Args, follow: bool) -> u64 {
    let named = match at::resolve_at(guest, AT_FDCWD, path) {
        Ok(named) => named,
        Err(e) => return errno::fail(e),
    };
    let full = walk::follow(guest, named, follow);
    if let Err(e) = meta::meta_of(guest, full.clone(), false) {
        return errno::fail(e);
    }
    answer(guest, &full, op, args)
}

pub fn by_fd(guest: &Guest, fd: u64, op: Op, args: Args) -> u64 {
    match guest.fds.get(fd as usize).filter(|f| f.is_open()) {
        Some(f) if matches!(f.kind, Kind::File | Kind::Dir) => {
            answer(guest, &f.path.clone(), op, args)
        }
        /* A pipe, a socket: nothing a guest names has attributes there. */
        Some(_) => errno::fail(errno::EOPNOTSUPP),
        None => errno::fail(errno::EBADF),
    }
}
