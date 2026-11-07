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

//! Which chunk goes on which stream: up to WIDTH_MAX streams, each with at
//! most one chunk in flight, the lowest chunk not held and not in flight
//! first. A chunk whose stream broke is asked again on another stream when
//! one is free, and given up after TRIES asks; the install then stops and
//! keeps what came. Pure, so the host proofs hold it.

use alloc::vec;
use alloc::vec::Vec;

/// The most streams an install holds at once (nonos_route_link
/// INSTALL_STREAMS; the project allows four).
pub const WIDTH_MAX: usize = 4;
/// Asks for one chunk before it is given up.
pub const TRIES: u8 = 4;

pub struct Sched {
    held: Vec<bool>,
    tries: Vec<u8>,
    /// The stream a chunk last failed on, so it goes to another.
    failed_on: Vec<Option<usize>>,
    slots: Vec<Option<usize>>,
}

impl Sched {
    /// `held` says which chunks are kept already; `width` is capped at
    /// WIDTH_MAX and is at least one.
    pub fn new(held: Vec<bool>, width: usize) -> Self {
        let n = held.len();
        Sched { held, tries: vec![0; n], failed_on: vec![None; n], slots: vec![None; width.clamp(1, WIDTH_MAX)] }
    }

    pub fn width(&self) -> usize {
        self.slots.len()
    }

    /// The chunks to ask for at this step, each with the free stream it goes on.
    pub fn assign(&mut self) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for slot in 0..self.slots.len() {
            if self.slots[slot].is_some() {
                continue;
            }
            let waiting = |c: &usize| !self.held[*c] && self.tries[*c] < TRIES && !self.slots.contains(&Some(*c));
            let elsewhere = (0..self.held.len()).filter(waiting).find(|c| self.failed_on[*c] != Some(slot));
            // A chunk that failed here goes here again only when no other stream is free.
            let others_free = self.slots.iter().enumerate().any(|(s, c)| s != slot && c.is_none());
            let any = (0..self.held.len()).find(|c| waiting(c));
            let pick = elsewhere.or(if others_free { None } else { any });
            if let Some(c) = pick {
                self.slots[slot] = Some(c);
                self.tries[c] += 1;
                out.push((slot, c));
            }
        }
        out
    }

    /// Stream `slot`'s chunk came whole and held to its digest, or did not.
    pub fn done(&mut self, slot: usize, ok: bool) {
        let Some(c) = self.slots.get_mut(slot).and_then(Option::take) else { return };
        match ok {
            true => self.held[c] = true,
            false => self.failed_on[c] = Some(slot),
        }
    }

    /// Every chunk held.
    pub fn whole(&self) -> bool {
        self.held.iter().all(|h| *h)
    }

    /// Nothing in flight and nothing more to ask: whole, or a chunk given up.
    pub fn over(&self) -> bool {
        self.slots.iter().all(Option::is_none)
            && (0..self.held.len()).all(|c| self.held[c] || self.tries[c] >= TRIES)
    }

    /// The chunk on stream `slot`, if one is in flight there.
    pub fn on(&self, slot: usize) -> Option<usize> {
        self.slots.get(slot).copied().flatten()
    }
}
