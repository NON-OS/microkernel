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

use super::pick::pick;
use core::sync::atomic::{AtomicU32, Ordering};

pub static LAST_SCHEDULED_PID: AtomicU32 = AtomicU32::new(0);

pub(super) static LAST_PER_BAND: [AtomicU32; 5] =
    [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];

/// Pick a process to run next and claim it, so two CPUs cannot pick the same
/// one. Picking only reads state and `Running` is not set until the arch
/// switch, so without the claim both would switch into the same control block.
/// Callers either switch to what they get back or leave it `Running`, so
/// claiming early does not strand anything.
pub fn select_next_process() -> Option<u32> {
    // Bounded: a lost claim means the state moved, so the next pick sees it.
    // The cap stops a churn spinning here with interrupts off.
    const CLAIM_ATTEMPTS: usize = 8;
    for _ in 0..CLAIM_ATTEMPTS {
        let (pid, band) = pick()?;
        if super::claim::claim(pid) {
            if let Some(idx) = band {
                LAST_PER_BAND[idx].store(pid, Ordering::Relaxed);
            }
            LAST_SCHEDULED_PID.store(pid, Ordering::Relaxed);
            return Some(pid);
        }
    }
    None
}
