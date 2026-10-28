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

/*
 * The family's memfd seals, by the identity every dup of a memfd shares
 * (fd_make.rs). A memfd with no entry was made without MFD_ALLOW_SEALING
 * and carries SEAL alone.
 */

use alloc::vec::Vec;
use core::cell::RefCell;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::memfd_seal_rules::{add, first_seals, F_GET_SEALS, SEAL};
use crate::linux::file::memfd_id;

pub(super) struct Seals(pub(super) RefCell<Vec<(u32, u32)>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Seals {}

pub(super) static SEALS: Seals = Seals(RefCell::new(Vec::new()));

/* The seals on the memfd `fd` names, or none when it names no memfd. */
pub fn seals_of(guest: &Guest, fd: u64) -> Option<u32> {
    let id = memfd_id(guest, fd)?;
    Some(SEALS.0.borrow().iter().find(|(x, _)| *x == id).map_or(SEAL, |(_, s)| *s))
}

/* The memfd memfd_create made as `got` starts with the seals its flags say. */
pub fn seals_start(guest: &Guest, got: u64, flags: u64) {
    match memfd_id(guest, got) {
        Some(id) if first_seals(flags) != SEAL => {
            SEALS.0.borrow_mut().push((id, first_seals(flags)))
        }
        _ => {}
    }
}

/* F_ADD_SEALS and F_GET_SEALS; anything but a memfd is EINVAL, as on Linux. */
pub fn fcntl_seal(guest: &Guest, fd: u64, cmd: u64, arg: u64) -> u64 {
    let (Some(id), Some(now)) = (memfd_id(guest, fd), seals_of(guest, fd)) else {
        return errno::fail(errno::EINVAL);
    };
    if cmd == F_GET_SEALS {
        return errno::ok(u64::from(now));
    }
    let seals = match add(now, arg, super::memfd_seal_maps::writably_mapped(guest, id)) {
        Ok(seals) => seals,
        Err(e) => return errno::fail(e),
    };
    let mut all = SEALS.0.borrow_mut();
    all.retain(|(x, _)| *x != id);
    all.push((id, seals));
    errno::ok(0)
}
