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

//! Owners register, the compositor (pid 0) attaches, owners exit, the
//! compositor lets go: the exit path's frame handling, counted per frame.

use std::collections::BTreeMap;

use super::register_rule::may_register;

/// Owner `p` has its own pages p*100.. and DMA pages p*100+50.. (the broker's).
fn page(p: u64, i: u64, device: bool) -> u64 {
    p * 100 + i + if device { 50 } else { 0 }
}

#[derive(Default)]
pub struct World {
    pub slots: Vec<(u64, Vec<u64>, bool)>, // owner, frames, attached
    orphans: Vec<Vec<u64>>,
    pub freed: BTreeMap<u64, u32>,
}

impl World {
    pub fn register(&mut self, rule: bool, p: u64, at: u64, n: u64, device: bool) {
        let mut new: Vec<u64> = (at..at + n).map(|i| page(p, i, device)).collect();
        new.sort_unstable();
        let held = self.slots.iter().flat_map(|s| s.1.clone()).chain(self.orphans.concat());
        if !rule || may_register(device, &new, held).is_ok() {
            self.slots.push((p, new, false));
        }
    }
    fn free(&mut self, f: u64) {
        *self.freed.entry(f).or_default() += 1;
    }
    /// keep_at_exit, then the broker's DMA release, then the table teardown.
    pub fn exit(&mut self, p: u64) {
        let (mine, rest): (Vec<_>, Vec<_>) = self.slots.drain(..).partition(|s| s.0 == p);
        self.slots = rest;
        let kept: Vec<u64> = mine
            .into_iter()
            .filter(|s| s.2)
            .flat_map(|s| {
                self.orphans.push(s.1.clone());
                s.1
            })
            .collect();
        (0..8).for_each(|i| self.free(page(p, i, true)));
        (0..8)
            .filter(|i| !kept.contains(&page(p, *i, false)))
            .for_each(|i| self.free(page(p, i, false)));
    }
    pub fn compositor_lets_go(&mut self) {
        for f in self.orphans.concat() {
            self.free(f);
        }
        self.orphans.clear();
    }
}
