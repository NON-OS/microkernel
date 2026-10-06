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

//! What the kernel holds for this window: the capability word in its process
//! table row, and its entry in the attestation registry. Read once when the
//! window opens; neither changes while a process lives.

use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::admission::{self, Admission};
use super::verify::{attested, each};

/// The capability word the kernel recorded for this pid. `None` when the
/// process table did not answer or does not list this window.
pub fn held_mask() -> Option<u64> {
    let me = mk_getpid();
    if me == 0 {
        return None;
    }
    let mut held = None;
    let answered = each(|e| {
        if e.pid == me {
            held = Some(e.caps);
        }
    });
    if answered {
        held
    } else {
        None
    }
}

pub fn admission() -> Admission {
    let entries: Option<Vec<(u32, u8)>> =
        attested().map(|list| list.iter().map(|a| (a.pid, a.authority)).collect());
    admission::of(mk_getpid(), entries.as_deref())
}
