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

//! What select asks of a descriptor, and a set cleared before it is filled.

use alloc::vec;

use super::super::ready_sets::{select_count, set_bytes};
use super::super::{POLLERR, POLLHUP};
use crate::linux::file::MAX_FDS;
use crate::linux::guest::Guest;

/// What makes a descriptor count as ready in each set, as Linux's select
/// counts it: end of file is readable, and an error is both.
pub(super) const POLLIN: u16 = 0x001 | POLLHUP | POLLERR;
pub(super) const POLLOUT: u16 = 0x004 | POLLERR;

/// The sets as a select whose time ran out leaves them: empty.
pub fn clear(guest: &mut Guest, nfds: u64, sets: [u64; 3]) {
    let Ok(nfds) = select_count(nfds, MAX_FDS as u64) else {
        return;
    };
    let empty = vec![0u8; set_bytes(nfds)];
    for at in sets.into_iter().filter(|&at| at != 0) {
        let _ = guest.write(at, &empty);
    }
}
