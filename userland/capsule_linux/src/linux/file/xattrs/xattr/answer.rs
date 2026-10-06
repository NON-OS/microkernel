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

/* One xattr call answered from the family's table. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::{cache, cstr, resolve, synth};
use super::super::xattr_table as table;
use super::calls::{Args, Op};
use super::give::give;

pub(super) fn answer(guest: &Guest, path: &[u8], op: Op, a: Args) -> u64 {
    if synth::owns(path) {
        return match op {
            Op::List => errno::ok(0),
            _ => errno::fail(errno::EOPNOTSUPP),
        };
    }
    if let Op::List = op {
        return give(guest, a.value, a.size, table::list(path));
    }
    let name = match cstr::read_cstr(guest, a.name, 256) {
        Some(name) => name,
        None => return errno::fail(errno::EFAULT),
    };
    if let Err(e) = table::check_name(&name) {
        return errno::fail(e);
    }
    let writable = cache::held(path) || resolve::key(path).writable().is_ok();
    let done = match op {
        Op::Get => {
            return match table::get(path, &name) {
                Ok(value) => give(guest, a.value, a.size, value),
                Err(e) => errno::fail(e),
            };
        }
        Op::Set if !writable => Err(errno::EROFS),
        Op::Remove if !writable => Err(errno::EROFS),
        Op::Set => match guest.read(a.value, (a.size as usize).min(65537)) {
            Some(value) => table::set(path, &name, value, a.flags),
            None => Err(errno::EFAULT),
        },
        Op::Remove => table::remove(path, &name),
        Op::List => Ok(()),
    };
    match done {
        Ok(()) => errno::ok(0),
        Err(e) => errno::fail(e),
    }
}
