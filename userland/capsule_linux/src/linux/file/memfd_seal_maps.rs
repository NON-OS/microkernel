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

/* What a memfd's seals refuse, and when the family lets them go. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::memfd_seal::{seals_of, SEALS};
use super::memfd_seal_rules::{resize_refused, writes_refused, F_ADD_SEALS, F_GET_SEALS};

const PROT_WRITE: u64 = 0x2;
const MAP_SHARED: u64 = 0x1;

pub fn is_seal_cmd(cmd: u64) -> bool {
    matches!(cmd, F_ADD_SEALS | F_GET_SEALS)
}

/* Whether memfd `id` is mapped writable now, through any descriptor on it. */
pub(super) fn writably_mapped(guest: &Guest, id: u32) -> bool {
    let at = |e: &crate::linux::guest::Fd| e.offset;
    let on = guest.fds.iter().filter(|e| e.kind == Kind::Memfd && e.handle == id && e.offset != 0);
    on.map(at).any(|a| guest.regions.iter().any(|r| r.write && r.at <= a && a < r.at + r.len))
}

/* Whether WRITE or FUTURE_WRITE refuses a write through `fd`. */
pub fn write_sealed(guest: &Guest, fd: u64) -> bool {
    seals_of(guest, fd).is_some_and(writes_refused)
}

/*
 * write and writev on a memfd. A sealed one is EPERM; this personality
 * keeps a memfd's bytes only in its mapping, so any other is EBADF.
 */
pub fn memfd_write(guest: &Guest, fd: u64) -> u64 {
    match write_sealed(guest, fd) {
        true => errno::fail(errno::EPERM),
        false => errno::fail(errno::EBADF),
    }
}

/* Whether a new mapping of `fd` with `prot` and `flags` is refused. */
pub fn map_sealed(guest: &Guest, fd: u64, prot: u64, flags: u64) -> bool {
    flags & MAP_SHARED != 0 && prot & PROT_WRITE != 0 && write_sealed(guest, fd)
}

/* Whether SHRINK or GROW refuses sizing the memfd `fd` from `size` to `to`. */
pub fn resize_sealed(guest: &Guest, fd: u64, size: u64, to: u64) -> bool {
    seals_of(guest, fd).is_some_and(|s| resize_refused(s, size, to))
}

/* Let go of the seals of every memfd no descriptor in the family names. */
pub fn seals_keep(named: impl Fn(u32) -> bool) {
    let mut all = SEALS.0.borrow_mut();
    if !all.is_empty() {
        all.retain(|(id, _)| named(*id));
    }
}
