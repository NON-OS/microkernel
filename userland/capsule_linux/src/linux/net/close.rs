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

//! Closing a socket descriptor. The socket goes with the last descriptor
//! naming it in the last process holding it.

use crate::linux::guest::{Guest, Kind};

use super::fd::sock_of;
use super::sock;

/// Called by close before it clears `fd`: this process lets go of the
/// socket unless another of its descriptors still names it.
pub fn close(guest: &mut Guest, fd: u64) {
    let Ok(id) = sock_of(guest, fd) else {
        return;
    };
    let named_again = guest
        .fds
        .iter()
        .enumerate()
        .any(|(i, f)| i as u64 != fd && f.kind == Kind::Socket && f.handle == id);
    if !named_again {
        sock::with(|t| t.release(id, guest.pid));
    }
}

/// Close a descriptor this module opened and cannot hand out after all.
pub(super) fn discard(guest: &mut Guest, fd: u64) {
    close(guest, fd);
    let _ = crate::linux::file::close(guest, fd);
}
