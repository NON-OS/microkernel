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

//! The owner's exit on the model: exit_owned.rs `orphan_owned`, run under
//! the gate before any of the exiting group's grants is torn down.

use super::common::{foreign, Orphan, Registry};
use super::dma_rule::strip;

impl Registry {
    /// Every surface `group` owns loses its frames; one another process
    /// still maps becomes an orphan. `dma`: (holder, start, len) of every
    /// grant (dma::grant_ranges); `same(holder)`: a thread of `group`.
    pub fn exit(&mut self, group: u32, dma: &[(u32, u64, u64)], same: impl Fn(u32) -> bool) {
        let dma: Vec<_> = dma.iter().filter(|g| same(g.0)).collect();
        let attaches = self.attaches.clone();
        for s in self.slots.iter_mut().filter(|s| s.owner == group) {
            let frames = core::mem::take(&mut s.frames);
            if frames.is_empty() || !foreign(&attaches, s.handle, group) {
                continue;
            }
            let mut free = if s.grant.is_some() { Vec::new() } else { frames.clone() };
            for &&(_, start, len) in &dma {
                strip(&mut free, |f| *f, start, len);
            }
            let (handle, grant) = (s.handle, s.grant);
            self.orphans.push(Orphan { owner: group, handle, frames, free, grant });
        }
    }
}
