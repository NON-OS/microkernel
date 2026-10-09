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

use alloc::vec::Vec;

use crate::memory::addr::PhysAddr;
use crate::smp::lock_responsive;

use super::claim_owned::{claim_owned, foreign_windows};
use super::gate::GATE;
use super::orphans::{reclaim, Orphan, ORPHANS};
use super::window::Window;

/*
 * The surface frames an munmap of `pid` must not free: frames of surfaces
 * `pid` attached from another owner, and frames of its own surfaces that
 * another process still has attached.
 */
pub struct Held {
    windows: Vec<Window>,
}

impl Held {
    pub fn collect(pid: u32, addr: u64, len: u64) -> Self {
        let end = addr.saturating_add(len);
        let mut windows = Vec::new();
        let _gate = lock_responsive(&GATE);
        claim_owned(pid, addr, end, &mut windows);
        foreign_windows(pid, addr, end, &mut windows);
        Self { windows }
    }

    /* True when the frame `pa` just unmapped at `va` must stay allocated. */
    pub fn holds(&mut self, va: u64, pa: PhysAddr) -> bool {
        let pa = PhysAddr::new(pa.as_u64() & !0xFFF);
        for w in self.windows.iter_mut() {
            let idx = (va.wrapping_sub(w.base) / 4096) as usize;
            if va >= w.base && w.frames.get(idx) == Some(&pa) {
                if w.orphan.is_some() {
                    w.unmapped.push(pa);
                }
                return true;
            }
        }
        false
    }

    /*
     * Hand the frames the owner really unmapped to the registry, which
     * frees them once no other process maps them. A frame whose PTE could
     * not be removed stays allocated for good.
     */
    pub fn settle(self) {
        for w in self.windows {
            if let Some((owner, handle)) = w.orphan {
                ORPHANS.lock().push(Orphan { owner, handle, frames: w.frames, free: w.unmapped });
            }
        }
        reclaim();
    }
}
