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

//! An event whose method the budget keeps cutting short (F-126). Cut
//! `CUTS` runs in a row, it is left off for a window, then armed again; a
//! run cut again at once sends it off for twice as long, up to `LAST_MS`.
//! A method that never finishes so costs at most one budget per window,
//! and one that recovers is served again within `LAST_MS`. Any run that
//! finishes clears the count and the window. Our bounds, from no source.

use super::storm::Storms;
use alloc::vec::Vec;

pub const CUTS: u8 = 3;
pub const FIRST_MS: u64 = 1000;
pub const LAST_MS: u64 = 64_000;

impl Storms {
    /// After a run of `key` at `now`: Some(window) when it is now to be left
    /// off for that long. A run that failed on its own changes nothing.
    pub fn after_run(&mut self, key: u16, finished: bool, cut: bool, now: u64) -> Option<u64> {
        if finished {
            self.cuts.remove(&key);
            self.windows.remove(&key);
            return None;
        }
        if !cut {
            return None;
        }
        let n = self.cuts.entry(key).or_insert(0);
        *n = n.saturating_add(1);
        if *n < CUTS {
            return None;
        }
        let w = self.windows.get(&key).map_or(FIRST_MS, |w| w.saturating_mul(2).min(LAST_MS));
        self.windows.insert(key, w);
        self.off.insert(key, now.saturating_add(w));
        Some(w)
    }

    /// The events whose window has passed at `now`, no longer off.
    pub fn due(&mut self, now: u64) -> Vec<u16> {
        let due: Vec<u16> = self.off.iter().filter(|(_, t)| **t <= now).map(|(k, _)| *k).collect();
        due.iter().for_each(|k| _ = self.off.remove(k));
        due
    }
}
