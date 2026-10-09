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

//! The Downloads page's list: every link the person asked for, in the order
//! asked, where each stands, and what can be done with it. Pure, so the
//! proofs hold it; the worker that does the downloading is `job.rs`, and one
//! download runs at a time (two over an onion route only halve each other).

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// The most rows kept. Past it the oldest finished ones go first.
pub const MAX_ROWS: usize = 64;
/// The speed is measured over at least this long, so a burst of buffered
/// bytes does not read as a fast link.
const SPEED_WINDOW_MS: i64 = 1_000;

/// Where one download stands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    /// Waiting for the one before it.
    Queued,
    Running,
    /// Asked to stop; the worker stops at its next read.
    Stopping,
    /// In the music folder at this path.
    Done(String),
    /// Stopped with this sentence. `resumable`: what came is kept, and the
    /// next try asks the server for the rest.
    Failed { why: &'static str, resumable: bool },
    /// Stopped by the person; what came is kept.
    Cancelled,
}

/// One row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub id: u32,
    /// The address as given, to fetch again.
    pub url: String,
    /// What the row is called: the file the address names.
    pub name: String,
    pub state: State,
    pub done: u64,
    pub total: Option<u64>,
    /// Bytes a second, smoothed; 0 until measured.
    pub speed: u64,
    /// Where the speed was last measured from.
    mark_ms: i64,
    mark_done: u64,
}

/// What can be done to a row, as its buttons offer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    Cancel,
    Resume,
    Play,
    Remove,
}

impl Row {
    /// The buttons this row shows, most likely first.
    pub fn acts(&self) -> &'static [Act] {
        match self.state {
            State::Queued | State::Running => &[Act::Cancel],
            State::Stopping => &[],
            State::Done(_) => &[Act::Play, Act::Remove],
            State::Failed { resumable: true, .. } | State::Cancelled => &[Act::Resume, Act::Remove],
            // Trying again would get the same answer: not an MP3, too large.
            State::Failed { resumable: false, .. } => &[Act::Remove],
        }
    }

    /// Whole seconds left at the measured speed, when both are known.
    pub fn eta_secs(&self) -> Option<u64> {
        let total = self.total?;
        if self.speed == 0 || self.state != State::Running {
            return None;
        }
        Some(total.saturating_sub(self.done).div_ceil(self.speed))
    }

    fn finished(&self) -> bool {
        matches!(self.state, State::Done(_) | State::Failed { .. } | State::Cancelled)
    }
}

#[derive(Default)]
pub struct Downloads {
    rows: Vec<Row>,
    next_id: u32,
}

impl Downloads {
    pub const fn new() -> Downloads {
        Downloads { rows: Vec::new(), next_id: 1 }
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn get(&self, id: u32) -> Option<&Row> {
        self.rows.iter().find(|r| r.id == id)
    }

    fn get_mut(&mut self, id: u32) -> Option<&mut Row> {
        self.rows.iter_mut().find(|r| r.id == id)
    }

    /// Ask for `url`, called `name`. Its id, or None when the same address
    /// is already waiting or running.
    pub fn add(&mut self, url: &str, name: &str) -> Option<u32> {
        let busy = |r: &Row| matches!(r.state, State::Queued | State::Running | State::Stopping);
        if self.rows.iter().any(|r| r.url == url && busy(r)) {
            return None;
        }
        if self.rows.len() >= MAX_ROWS {
            let oldest = self.rows.iter().position(Row::finished)?;
            self.rows.remove(oldest);
        }
        let id = self.next_id.max(1);
        self.next_id = id.wrapping_add(1).max(1);
        self.rows.push(Row {
            id,
            url: String::from(url),
            name: String::from(name),
            state: State::Queued,
            done: 0,
            total: None,
            speed: 0,
            mark_ms: 0,
            mark_done: 0,
        });
        Some(id)
    }

    /// The download running, or stopping, if any.
    pub fn active(&self) -> Option<u32> {
        let on = |r: &&Row| matches!(r.state, State::Running | State::Stopping);
        self.rows.iter().find(on).map(|r| r.id)
    }

    /// The next to start when none is running: the first queued. Marked
    /// running.
    pub fn start_next(&mut self, now_ms: i64) -> Option<&Row> {
        if self.active().is_some() {
            return None;
        }
        let row = self.rows.iter_mut().find(|r| r.state == State::Queued)?;
        row.state = State::Running;
        row.speed = 0;
        row.mark_ms = now_ms;
        row.mark_done = row.done;
        Some(row)
    }

    /// The worker's progress for `id` at `now_ms`.
    pub fn progress(&mut self, id: u32, done: u64, total: Option<u64>, now_ms: i64) {
        let Some(r) = self.get_mut(id) else { return };
        r.done = done;
        if total.is_some() {
            r.total = total;
        }
        let span = now_ms.saturating_sub(r.mark_ms);
        if done < r.mark_done {
            // The server started the file again from its first byte.
            r.mark_done = done;
            r.mark_ms = now_ms;
        } else if span >= SPEED_WINDOW_MS {
            let now = (done - r.mark_done).saturating_mul(1000) / span as u64;
            r.speed = if r.speed == 0 { now } else { (r.speed * 3 + now) / 4 };
            r.mark_done = done;
            r.mark_ms = now_ms;
        }
    }

    /// The worker ended `id`: in the music folder at `Ok(path)`, or stopped
    /// with `Err((why, resumable))`. A row the person cancelled stays
    /// cancelled whatever the worker said last.
    pub fn ended(&mut self, id: u32, outcome: Result<String, (&'static str, bool)>) {
        let Some(r) = self.get_mut(id) else { return };
        r.speed = 0;
        r.state = match (outcome, &r.state) {
            (Ok(path), _) => State::Done(path),
            (Err(_), State::Stopping) => State::Cancelled,
            (Err((why, resumable)), _) => State::Failed { why, resumable },
        };
    }

    /// Cancel `id`. True when the worker has to be told: it is running.
    pub fn cancel(&mut self, id: u32) -> bool {
        let Some(r) = self.get_mut(id) else { return false };
        match r.state {
            State::Queued => {
                r.state = State::Cancelled;
                false
            }
            State::Running => {
                r.state = State::Stopping;
                true
            }
            _ => false,
        }
    }

    /// Queue `id` again, to go on from what came. True when it was one
    /// that can be: cancelled, or stopped by the network.
    pub fn resume(&mut self, id: u32) -> bool {
        let Some(r) = self.get_mut(id) else { return false };
        if !matches!(r.state, State::Cancelled | State::Failed { resumable: true, .. }) {
            return false;
        }
        r.state = State::Queued;
        true
    }

    /// Take `id` off the list, unless it is running.
    pub fn remove(&mut self, id: u32) -> bool {
        let Some(at) = self.rows.iter().position(|r| r.id == id) else { return false };
        if !self.rows[at].finished() {
            return false;
        }
        self.rows.remove(at);
        true
    }

    /// Take every finished row off.
    pub fn clear_finished(&mut self) {
        self.rows.retain(|r| !r.finished());
    }
}
