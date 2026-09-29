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

//! `prctl`, for what glibc and runtimes ask of it: a thread's name, and the
//! two flags whose honest answer this machine already gives.

use alloc::collections::BTreeMap;
use core::cell::RefCell;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const PR_SET_NAME: u64 = 15;
const PR_GET_NAME: u64 = 16;
const PR_SET_NO_NEW_PRIVS: u64 = 38;
const PR_GET_NO_NEW_PRIVS: u64 = 39;
const PR_GET_DUMPABLE: u64 = 3;
const PR_SET_DUMPABLE: u64 = 4;
const NAME_LEN: usize = 16;

// Per thread in this family, so a name set on one thread is not another's.
// The personality answers one trap at a time on one thread, so a RefCell.
struct Names(RefCell<BTreeMap<u32, [u8; NAME_LEN]>>);
// SAFETY: eK@nonos.systems - only the personality's single serve loop touches it.
unsafe impl Sync for Names {}
static NAMES: Names = Names(RefCell::new(BTreeMap::new()));

pub fn prctl(guest: &Guest, tid: u32, option: u64, arg: u64) -> u64 {
    match option {
        PR_SET_NAME => match guest.read(arg, NAME_LEN) {
            Some(raw) => {
                let mut name = [0u8; NAME_LEN];
                let end = raw.iter().position(|b| *b == 0).unwrap_or(NAME_LEN - 1);
                name[..end.min(NAME_LEN - 1)].copy_from_slice(&raw[..end.min(NAME_LEN - 1)]);
                NAMES.0.borrow_mut().insert(tid, name);
                errno::ok(0)
            }
            None => errno::fail(errno::EFAULT),
        },
        PR_GET_NAME => {
            let name = NAMES.0.borrow().get(&tid).copied().unwrap_or([0u8; NAME_LEN]);
            match guest.write(arg, &name) == NAME_LEN as i64 {
                true => errno::ok(0),
                false => errno::fail(errno::EFAULT),
            }
        }
        // Nothing here ever raises privilege, so no-new-privs already holds.
        PR_SET_NO_NEW_PRIVS if arg == 1 => errno::ok(0),
        PR_GET_NO_NEW_PRIVS => errno::ok(1),
        // No core is ever written, so a guest is not dumpable and cannot be made so.
        PR_GET_DUMPABLE => errno::ok(0),
        PR_SET_DUMPABLE if arg == 0 => errno::ok(0),
        _ => errno::fail(errno::EINVAL),
    }
}
