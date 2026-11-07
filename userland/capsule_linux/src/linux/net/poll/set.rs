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


//! Walking a guest's `struct pollfd` array.

use crate::linux::abi::errno;
use crate::linux::file::MAX_FDS;
use crate::linux::guest::Guest;

use super::super::ready_sets::poll_count;
use super::answer::{answer, POLLFD_LEN};
use super::ready::ready;

/// The `count` entries at `at`, each answered in place. A count above
/// RLIMIT_NOFILE is EINVAL, as Linux has it (`ready_sets`), so one call never
/// walks more than a guest can have open. The array is read in one copy and
/// written back in one, every revents with it, zero or not, as Linux writes
/// them: two copies a descriptor made every retry of a parked poll cost two
/// trips into the guest's memory each.
pub fn poll(guest: &mut Guest, at: u64, count: u64) -> u64 {
    let count = match poll_count(count, MAX_FDS as u64) {
        Ok(n) => n,
        Err(e) => return errno::fail(e),
    };
    let Some(len) = (count as usize).checked_mul(POLLFD_LEN) else {
        return errno::fail(errno::EFAULT);
    };
    if len == 0 {
        return errno::ok(0);
    }
    let Some(mut raw) = guest.read(at, len) else {
        return errno::fail(errno::EFAULT);
    };
    let hits = answer(&mut raw, |fd| ready(guest, fd));
    if guest.write(at, &raw) < len as i64 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(hits)
}
