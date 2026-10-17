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

//! An event that will not stop: a level GPE whose source keeps asserting
//! after its method ran, an edge one that fires without end, or an EC that
//! raises query after query. Each is counted per key in a window, and past
//! `RUNS` in `WINDOW_MS` the caller turns it off. Our bounds, from no
//! source: no real source fires two hundred times a second for long.

use alloc::collections::BTreeMap;

pub const RUNS: u32 = 200;
pub const WINDOW_MS: u64 = 1000;
/// Keys tracked at once; more GPEs than any machine has.
const KEYS: usize = 512;

#[derive(Debug, Default)]
pub struct Storms {
    /// Per key: the window's start and the runs in it.
    seen: BTreeMap<u16, (u64, u32)>,
    /// Per key: runs in a row the budget cut short, the back-off window
    /// last used, and when an event left off is armed again (`storm_backoff`).
    pub(super) cuts: BTreeMap<u16, u8>,
    pub(super) windows: BTreeMap<u16, u64>,
    pub(super) off: BTreeMap<u16, u64>,
}

impl Storms {
    /// Counts one run of `key` at `now`; true once it is past the bound.
    pub fn hit(&mut self, key: u16, now: u64) -> bool {
        if !self.seen.contains_key(&key) && self.seen.len() >= KEYS {
            return true;
        }
        let e = self.seen.entry(key).or_insert((now, 0));
        if now.saturating_sub(e.0) >= WINDOW_MS {
            *e = (now, 0);
        }
        e.1 += 1;
        e.1 > RUNS
    }
}
