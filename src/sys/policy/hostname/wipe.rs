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

//! Forgetting the names at the end of a boot. They are statics, outside the
//! heap, so the heap erase never reaches them, and the person typed them.
//! The other CPUs are parked by then, possibly holding the lock, so it is
//! tried once and never waited on; a held lock is said, and the names stay.

use super::names_clear::clear;
use super::state::STATE;

/// Zero the host and domain names. Takes no lock it cannot get at once and
/// allocates nothing, so it may run inside the shutdown wipe.
pub fn wipe() {
    let Some(mut s) = STATE.try_lock() else {
        crate::sys::serial::print(b"[ZS] host name not wiped: its lock is held\n");
        return;
    };
    let s = &mut *s;
    clear(&mut s.hostname, &mut s.hostname_len);
    clear(&mut s.domainname, &mut s.domainname_len);
}
