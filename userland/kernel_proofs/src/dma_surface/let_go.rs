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

//! A holder's let-go on the model: drop_attach, then orphans.rs `reclaim`,
//! which runs orphans_free.rs `free_orphans` under GATE and grant_run.rs
//! `free_unheld` once GATE is let go.

use super::common::{foreign, Registry};

impl Registry {
    /// `pid` lets go of `handle`; then the reclaim every let-go runs.
    pub fn drop_attach(&mut self, pid: u32, handle: u32) {
        self.attaches.retain(|&(h, p)| !(h == handle && p == pid));
        self.free_orphans();
        self.free_unheld();
    }

    /// The orphans no other process maps go, their free lists freed.
    pub fn free_orphans(&mut self) {
        let attaches = self.attaches.clone();
        let all = core::mem::take(&mut self.orphans);
        let (kept, done): (Vec<_>, Vec<_>) =
            all.into_iter().partition(|o| foreign(&attaches, o.handle, o.owner));
        self.orphans = kept;
        for o in done {
            self.slots.retain(|s| s.handle != o.handle);
            self.pages_freed.extend(o.free);
        }
    }

    /// Every run no orphan holds any more goes back, each once.
    pub fn free_unheld(&mut self) {
        let held: Vec<(u64, u64)> = self.orphans.iter().filter_map(|o| o.grant).collect();
        let given = &mut self.given_back;
        self.runs.retain(|run| {
            let keep = held.contains(run);
            if !keep {
                given.push(*run);
            }
            keep
        });
    }
}
