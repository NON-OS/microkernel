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

//! What each family keeps to itself.
//!
//! The Linux tree is one tree for every guest on the machine. A scratch
//! directory in it would be a name two guests share, and a file left there a
//! message from one to the other. These prefixes live instead under a root of
//! the family's own, named by a random id and outside `/linux`, so no path a
//! guest can write reaches another family's, and it is cleared at the end.

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub const PRIVATE: &[&[u8]] = &[b"/tmp", b"/dev/shm", b"/home", b"/root", b"/run", b"/var/tmp"];
const BASE: &[u8] = b"/linux-private/";

static ID: AtomicU64 = AtomicU64::new(0);
static INSTALLING: AtomicBool = AtomicBool::new(false);

/// Draw this family's id from the kernel's source. False if it cannot.
pub fn choose() -> bool {
    let mut id = [0u8; 8];
    if nonos_libc::crypto_random(id.as_mut_ptr(), id.len()) < 0 {
        return false;
    }
    ID.store(u64::from_le_bytes(id), Ordering::Relaxed);
    true
}

/// The store root this family's private prefixes live under.
pub fn root() -> Vec<u8> {
    let mut out = Vec::from(BASE);
    out.extend_from_slice(alloc::format!("{:016x}", ID.load(Ordering::Relaxed)).as_bytes());
    out
}

/// True when `visible` is one of the private prefixes or below one.
pub fn is_private(visible: &[u8]) -> bool {
    PRIVATE
        .iter()
        .any(|p| visible.starts_with(p) && matches!(visible.get(p.len()), None | Some(b'/')))
}

/// Only the install path writes the shared tree; a running guest never does.
pub fn allow_shared_writes() {
    INSTALLING.store(true, Ordering::Relaxed);
}

pub fn shared_writes_allowed() -> bool {
    INSTALLING.load(Ordering::Relaxed)
}
