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

//! DMA runs kept until their device's wipe has ended (L78), or for good when
//! the device may still reach them: an unconfined grant of a PCI function,
//! drained by a release. Scrubbed when kept, again when freed; `for_good`
//! marks one whose wipe ended so, and only those count against the cap.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::kept_rule::{on_let_go, Act, Run};

pub(super) struct Kept {
    pub(super) device_id: u64,
    pub(super) epoch: u64,
    pub(super) start: u64,
    pub(super) len: u64,
    pub(super) run: Run,
    pub(super) for_good: bool,
}

pub(super) static KEPT: Mutex<Vec<Kept>> = Mutex::new(Vec::new());

/// Held by the registry too until it says otherwise through `defer`.
pub(super) fn keep(device_id: u64, epoch: u64, start: u64, len: u64) {
    let (run, for_good) = (Run { let_go: false, settled: false }, false);
    KEPT.lock().push(Kept { device_id, epoch, start, len, run, for_good });
}

/// The registry let go of a run: None if not kept, else whether it waits.
pub(super) fn defer(start: u64, len: u64) -> Option<bool> {
    let mut all = KEPT.lock();
    let at = all.iter().position(|k| (k.start, k.len) == (start, len))?;
    let (run, act) = on_let_go(all[at].run);
    if act == Act::Free {
        all.remove(at);
        return Some(false);
    }
    all[at].run = run;
    Some(true)
}

pub(in crate::hardware::broker) fn kept_bytes(device_id: u64) -> u64 {
    let all = KEPT.lock();
    all.iter().filter(|k| k.device_id == device_id && k.for_good).map(|k| k.len).sum()
}

/// The shutdown wipe's share: every kept run zeroed, waiting no longer than `again` allows.
pub(super) fn scrub_all(mut again: impl FnMut() -> bool) -> bool {
    loop {
        if let Some(all) = KEPT.try_lock() {
            all.iter().for_each(|k| super::scrub::scrub(k.start, k.len));
            return true;
        }
        if !again() {
            return false;
        }
    }
}
