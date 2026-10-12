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

//! A registry for the tests: attach records, and the held grants.

use super::dma_plan::hold_in;
use super::dma_rule::{due, DmaOrphan, PageOrphan};

pub const GPU: u32 = 7;
pub const COMPOSITOR: u32 = 3;
pub const FB: u32 = 0x11;
/// The GPU's framebuffer grant: four pages at 1 MiB.
pub const START: u64 = 0x10_0000;
pub const LEN: u64 = 0x4000;

pub fn grant_frames() -> Vec<u64> {
    (0..4).map(|i| START + i * 0x1000).collect()
}

#[derive(Default)]
pub struct Registry {
    pub attaches: Vec<(u32, u32)>,
    /// (thread, process) for threads that are not their own process.
    pub threads: Vec<(u32, u32)>,
    pub orphans: Vec<PageOrphan<u32, u64>>,
    pub held: Vec<DmaOrphan<u32>>,
    pub given_back: Vec<(u64, u64)>,
}

impl Registry {
    /// dma_hold::hold_dma by `grant`'s pid; `live`: (handle, owner, frames)
    /// of the process's surfaces over the grant.
    pub fn release(&mut self, grant: u32, live: Vec<(u32, u32, Vec<u64>)>) -> bool {
        let attaches = self.attaches.clone();
        let threads = self.threads.clone();
        let group = |t: u32| threads.iter().find(|x| x.0 == t).map_or(t, |x| x.1);
        let foreign = |h, o| attaches.iter().any(|&(a, p): &(u32, u32)| a == h && p != o);
        let rules = (foreign, |a, b| group(a) == group(b));
        let kept = hold_in((grant, START, LEN), live, &mut self.orphans, rules, |f| *f);
        let held = kept.is_some();
        self.held.extend(kept);
        if !held {
            self.given_back.push((START, LEN));
        }
        held
    }

    /// drop_attach, then dma_held::reclaim_dma.
    pub fn detach(&mut self, pid: u32, handle: u32) {
        self.attaches.retain(|&(h, p)| !(h == handle && p == pid));
        let attaches = self.attaches.clone();
        let foreign = |h, o| attaches.iter().any(|&(a, p): &(u32, u32)| a == h && p != o);
        for h in due(&mut self.held, foreign) {
            self.given_back.push((h.start, h.len));
        }
    }
}
