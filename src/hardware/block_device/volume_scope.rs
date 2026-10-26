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

//! The data volume's own I/O, marked for the thread the kernel does it for
//! (its tid, which `current_pid` gives: a sibling thread of the same process
//! is not inside). Only crate::fs makes the token that opens a scope
//! (VolumeIo, whose field is private to fs::cryptoblock). A scope is let go
//! only by its guard's drop, on every path out of the I/O.

use core::sync::atomic::{AtomicU32, Ordering};

use super::volume_rule::{admits, free_slot, inside};
use crate::fs::cryptoblock::VolumeIo;
use crate::services::caps::{has_capability, CAP_DRIVER, CAP_STORAGE};

/// Threads inside volume I/O at once; past it a scope is refused.
const SLOTS: usize = 16;
static INSIDE: [AtomicU32; SLOTS] = [const { AtomicU32::new(0) }; SLOTS];

pub struct VolumeScope {
    slot: usize,
}

fn seen() -> [u32; SLOTS] {
    core::array::from_fn(|i| INSIDE[i].load(Ordering::Acquire))
}

/// A scope for the calling thread; None with no tid, or when every slot is
/// taken after at most SLOTS tries, never a wait.
pub fn volume_scope(_: &VolumeIo) -> Option<VolumeScope> {
    let tid = crate::process::current_pid()?;
    for _ in 0..SLOTS {
        let slot = free_slot(&seen())?;
        if INSIDE[slot].compare_exchange(0, tid, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            return Some(VolumeScope { slot });
        }
    }
    None
}

impl Drop for VolumeScope {
    fn drop(&mut self) {
        INSIDE[self.slot].store(0, Ordering::Release);
    }
}

/// The block clients' gate for thread `tid`: a driver, the store's holder,
/// or inside the data volume's own I/O.
pub fn disk_caller_admitted(tid: u32) -> bool {
    admits(inside(&seen(), tid), has_capability(tid, CAP_DRIVER), has_capability(tid, CAP_STORAGE))
}
