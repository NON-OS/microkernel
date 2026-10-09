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

//! Which wallpaper to fetch next, and what to do with one that came.
//!
//! The fetch and the decode run on a worker thread (`worker.rs`) so the
//! service keeps answering its own callers while a picture comes over, and
//! this is what the service thread keeps about it. One job at a time: the
//! policy may change its mind while a job runs, and the job is let finish
//! (it cannot be cut short mid call); what it brings is then dropped unless
//! it is still the one wanted, and the next job starts once it is in. A job
//! that stops partway hands back what came over, kept here for the next try
//! of that wallpaper. A failed wallpaper is tried again at the next poll of
//! the policy, not at once, so a catalog that cannot answer is not asked flat
//! out.
//!
//! Pure: `D` is what a stopped job hands back (the download), `I` what a
//! finished one does (the decoded picture). Held in wallpaper_catalog_proofs.

/// What a job came back with.
pub enum Outcome<D, I> {
    /// Fetched whole and decoded.
    Decoded(I),
    /// A call failed partway; what came over, for the next try.
    Stopped(D),
    /// The step named failed, with nothing worth keeping.
    Failed(&'static str),
}

pub struct Plan<D> {
    /// The wallpaper the policy last said it wants.
    wanted: Option<u8>,
    /// The wallpaper on the desktop, None while the built in one is.
    applied: Option<u8>,
    /// The wallpaper a job is out for.
    running: Option<u8>,
    /// A wallpaper that failed, not tried again until the next poll.
    held: Option<u8>,
    /// What came over of a wallpaper before its last job stopped.
    kept: Option<(u8, D)>,
}

impl<D> Default for Plan<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D> Plan<D> {
    pub const fn new() -> Self {
        Self { wanted: None, applied: None, running: None, held: None, kept: None }
    }

    /// A poll's answer: the policy wants `index`. Lifts the hold on a failed
    /// wallpaper, and lets go of bytes kept for one no longer wanted.
    pub fn want(&mut self, index: u8) {
        self.wanted = Some(index);
        self.held = None;
        if self.kept.as_ref().is_some_and(|(of, _)| *of != index) {
            self.kept = None;
        }
    }

    pub fn wanted(&self) -> Option<u8> {
        self.wanted
    }

    pub fn running(&self) -> Option<u8> {
        self.running
    }

    /// The job to start now, if any: the wallpaper wanted, when it is not on
    /// the desktop, no job is out and it is not held, with what came over of
    /// it before. The job is then out until `finish` or `not_started`.
    pub fn begin(&mut self) -> Option<(u8, Option<D>)> {
        let index = self.wanted?;
        if self.running.is_some() || self.applied == Some(index) || self.held == Some(index) {
            return None;
        }
        self.running = Some(index);
        let kept = match self.kept.take() {
            Some((of, d)) if of == index => Some(d),
            _ => None,
        };
        Some((index, kept))
    }

    /// The job `begin` gave was not started: its bytes come back. `hold`
    /// waits for the next poll before trying again (no worker could be had);
    /// without it the next tick tries (the last worker is not gone yet).
    pub fn not_started(&mut self, index: u8, kept: Option<D>, hold: bool) {
        if self.running == Some(index) {
            self.running = None;
        }
        if let Some(d) = kept {
            self.kept = Some((index, d));
        }
        if hold {
            self.held = Some(index);
        }
    }

    /// The job for `index` came back. The picture to paint when it decoded
    /// and is still wanted; otherwise None, keeping a stopped job's bytes and
    /// holding the wallpaper when it is still wanted, and dropping all of it
    /// when it is not (the next tick then starts the one wanted now).
    pub fn finish<I>(&mut self, index: u8, outcome: Outcome<D, I>) -> Option<I> {
        if self.running == Some(index) {
            self.running = None;
        }
        if self.wanted != Some(index) {
            return None;
        }
        match outcome {
            Outcome::Decoded(image) => return Some(image),
            Outcome::Stopped(d) => self.kept = Some((index, d)),
            Outcome::Failed(_) => {}
        }
        self.held = Some(index);
        None
    }

    /// The picture of `index` is on the desktop.
    pub fn shown(&mut self, index: u8) {
        self.applied = Some(index);
        self.held = None;
    }

    /// The picture of `index` came but could not be shown.
    pub fn missed(&mut self, index: u8) {
        self.held = Some(index);
    }
}
