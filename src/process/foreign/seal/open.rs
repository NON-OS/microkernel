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

//! A family's seal is reserved before its first guest exists, so a family
//! is either sealed whole or never starts.

use super::family::Family;
use super::identity::identity;
use super::nonce::{salt, trace_start};
use super::table::{with_family, LIVE};
use crate::syscall::microkernel::errnos::ERRNO_BUSY;

/// Never evicted: past this many, a new family's first guest is refused.
pub(crate) const MAX_LIVE: usize = 64;

/// Before `supervisor` makes a guest. A supervisor with guests already has
/// its family. Otherwise a family it left unsealed is sealed now, and a new
/// one is reserved; a full table or no randomness refuses the guest.
pub(crate) fn reserve(supervisor: u32) -> Result<(), i64> {
    if super::super::registry::guest_count(supervisor) != 0 {
        return Ok(());
    }
    // Two first spawns racing share one reservation.
    if with_family(supervisor, |f| f.enrolled) == Some(false) {
        return Ok(());
    }
    super::finish::finish(supervisor);
    let (Some(trace), Some(salt)) = (trace_start(), salt()) else {
        return Err(ERRNO_BUSY);
    };
    let mut live = LIVE.lock();
    if live.len() >= MAX_LIVE {
        return Err(ERRNO_BUSY);
    }
    live.push(Family::fresh(supervisor, identity(supervisor), trace, salt));
    Ok(())
}

/// A guest was recorded under `supervisor`: its family has begun.
pub(crate) fn enrolled(supervisor: u32) {
    with_family(supervisor, |f| f.enrolled = true);
}

pub(crate) fn started(supervisor: u32) {
    with_family(supervisor, |f| f.started = true);
}
