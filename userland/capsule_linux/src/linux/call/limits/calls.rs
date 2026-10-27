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
//! The limits this personality actually enforces.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::table::{limit_for, unchanged, RLIMIT};

pub fn getrlimit(guest: &Guest, resource: u64, out: u64) -> u64 {
    let Some((soft, hard)) = limit_for(resource) else {
        return errno::fail(errno::EINVAL);
    };
    let mut buf = [0u8; RLIMIT];
    buf[..8].copy_from_slice(&soft.to_le_bytes());
    buf[8..].copy_from_slice(&hard.to_le_bytes());
    match guest.write(out, &buf) {
        n if n < 0 => errno::fail(errno::EFAULT),
        _ => errno::ok(0),
    }
}

/// `prlimit64` reads and writes in one call. A new limit is taken only when
/// it is the limit already there: gpg sets RLIMIT_CORE to 0 before it holds
/// a key, and refusing that stopped it. Any other change is refused, since
/// nothing here would keep it.
pub fn prlimit64(guest: &Guest, resource: u64, new: u64, old: u64) -> u64 {
    if new != 0 {
        let Some(asked) = guest.read(new, RLIMIT) else {
            return errno::fail(errno::EFAULT);
        };
        if !unchanged(resource, &asked) {
            return errno::fail(errno::EPERM);
        }
    }
    if old == 0 {
        return errno::ok(0);
    }
    getrlimit(guest, resource, old)
}
