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
//! The GGTT confined (G-75), from the test's own reading: once a rig arms
//! it with the frames the driver holds, every present entry the driver
//! writes must name a page of one of them, and no entry may still name a
//! frame given back. Firmware's entries may be left or cleared, not written.

use super::gen9::Gen9;

/// An entry's page address, as the engine translates it.
pub const PAGE: u64 = 0x7F_FFFF_F000;

#[derive(Default)]
pub struct Confine {
    /// The IOVA ranges held; None until a rig arms the watch.
    pub held: Option<Vec<(u64, u64)>>,
    /// Entries written present naming a page not held: (index, entry).
    pub strays: Vec<(u32, u64)>,
    /// Entries still naming a frame when it was given back.
    pub stale: Vec<(u32, u64)>,
}

pub fn names(r: &[(u64, u64)], pte: u64) -> bool {
    r.iter().any(|&(a, b)| (a..b).contains(&(pte & PAGE)))
}

impl Gen9 {
    pub fn arm(&mut self, held: Vec<(u64, u64)>) {
        self.confine.held = Some(held);
    }

    pub fn hold(&mut self, r: (u64, u64)) {
        if let Some(h) = self.confine.held.as_mut() {
            h.push(r);
        }
    }

    /// Entry `i` written as `pte`, checked against what is held.
    pub fn put_entry(&mut self, i: u32, pte: u64) {
        self.ggtt[i as usize] = pte;
        let held = self.confine.held.as_deref();
        if pte & 1 != 0 && held.is_some_and(|h| !names(h, pte)) {
            self.confine.strays.push((i, pte));
        }
    }

    /// `ranges` given back: freed in the watch's eyes, no longer held, and
    /// any entry still naming one of them is stale.
    pub fn free(&mut self, ranges: impl IntoIterator<Item = (u64, u64)>) {
        let ranges: Vec<_> = ranges.into_iter().collect();
        if let Some(h) = self.confine.held.as_mut() {
            h.retain(|r| !ranges.contains(r));
            for (i, &pte) in self.ggtt.iter().enumerate() {
                if pte & 1 != 0 && names(&ranges, pte) {
                    self.confine.stale.push((i as u32, pte));
                }
            }
        }
        self.watch.freed.extend(ranges);
        self.check();
    }
}
