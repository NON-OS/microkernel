// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The data volume's own I/O, marked for the process the kernel does it
//! for. Only crate::fs makes the token that opens a scope (VolumeIo, whose
//! field is private to fs::cryptoblock), so no path a capsule reaches does;
//! dropped, the scope gives the process its own authority back.

use core::sync::atomic::{AtomicU32, Ordering};

use super::volume_rule::admits;
use crate::fs::cryptoblock::VolumeIo;
use crate::services::caps::{has_capability, CAP_DRIVER, CAP_STORAGE};

/// Processes inside volume I/O at once: more than the CPUs that run them.
const SLOTS: usize = 16;
static INSIDE: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];

pub struct VolumeScope {
    slot: usize,
}

/// A scope for the calling process; None with no pid or no free slot, and
/// the process is then held to its own capabilities.
pub fn volume_scope(_: &VolumeIo) -> Option<VolumeScope> {
    let pid = crate::process::current_pid()?;
    let ok = |i: &usize| {
        INSIDE[*i].compare_exchange(0, pid, Ordering::AcqRel, Ordering::Acquire).is_ok()
    };
    (0..SLOTS).find(ok).map(|slot| VolumeScope { slot })
}

impl Drop for VolumeScope {
    fn drop(&mut self) {
        INSIDE[self.slot].store(0, Ordering::Release);
    }
}

/// The block clients' gate for `pid`: a driver, the store's holder, or
/// inside the data volume's own I/O.
pub fn disk_caller_admitted(pid: u32) -> bool {
    let inside = INSIDE.iter().any(|s| s.load(Ordering::Acquire) == pid);
    admits(inside, has_capability(pid, CAP_DRIVER), has_capability(pid, CAP_STORAGE))
}
