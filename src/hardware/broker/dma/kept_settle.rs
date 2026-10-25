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

//! A function's kept runs settled as its wipe ends (L78): freed where the
//! end says nothing reaches them and the registry has let go, kept for good
//! where the device may still reach them, waiting for the registry else.

extern crate alloc;

use alloc::vec::Vec;

use super::kept::KEPT;
use super::kept_rule::{on_settle, Act};

/// Runs freed, and runs kept for good.
pub(in crate::hardware::broker) fn settle_kept(
    device_id: u64,
    epoch: u64,
    frees: bool,
) -> (usize, usize) {
    let (mut go, mut kept) = (Vec::new(), 0usize);
    KEPT.lock().retain_mut(|k| {
        if (k.device_id, k.epoch) != (device_id, epoch) {
            return true;
        }
        let (run, act) = on_settle(k.run, frees);
        k.run = run;
        match act {
            Act::Free => go.push((k.start, k.len)),
            Act::Keep => {
                k.for_good = true;
                kept += 1;
            }
            Act::Wait => {}
        }
        act != Act::Free
    });
    for &(start, len) in &go {
        super::held_run::free_now(start, len);
        crate::memory::iommu::note_unconfined_released(1);
    }
    (go.len(), kept)
}
