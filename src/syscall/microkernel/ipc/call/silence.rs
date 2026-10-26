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

//! A server silent to one caller, late calls only with no answer in time and
//! no late reply, for the kernel's default wait or longer is starved, however
//! short each call's own deadline. Kept per caller and endpoint in a fixed
//! table, no lock, no allocation. A full table evicts the pair silent longest
//! and reports it at once: an eviction makes a report earlier, never drops one.

pub(crate) const SLOTS: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Pair {
    pub(crate) caller: u32,
    pub(crate) port: u32,
}

/// A pair, when its silence began, and its last late call; `None` is free.
type Slot = (Option<Pair>, u64, u64);
type Told = [Option<Pair>; 2];

pub(crate) struct Silence {
    slots: [Slot; SLOTS],
    window_ms: u64,
}

impl Silence {
    pub(crate) const fn new(window_ms: u64) -> Self {
        Self { slots: [(None, 0, 0); SLOTS], window_ms }
    }

    /// The call `token` of `pair`, begun at `start`, came back late at `now`. Told:
    /// `pair` once silent the window (it then starts again), and any pair evicted.
    pub(crate) fn late(&mut self, pair: Pair, start: u64, now: u64, token: u64) -> Told {
        let mut out = [None, None];
        let at = self.slots.iter().position(|s| s.0 == Some(pair)).unwrap_or_else(|| {
            let oldest = || (0..SLOTS).min_by_key(|&i| self.slots[i].1).unwrap_or(0);
            let at = self.slots.iter().position(|s| s.0.is_none()).unwrap_or_else(oldest);
            out[1] = self.slots[at].0;
            self.slots[at] = (Some(pair), start, 0);
            at
        });
        self.slots[at].2 = token;
        if now.saturating_sub(self.slots[at].1) >= self.window_ms {
            self.slots[at].1 = now;
            out[0] = Some(pair);
        }
        out
    }

    /// `pair` was answered in time: it is not silent.
    pub(crate) fn answered(&mut self, pair: Pair) {
        self.slots.iter_mut().filter(|s| s.0 == Some(pair)).for_each(|s| s.0 = None);
    }

    /// `caller` found the late reply to its call `token`: the server it called
    /// answered after all, so that pair is not silent either.
    pub(crate) fn replied(&mut self, caller: u32, token: u64) {
        let theirs = |s: &&mut Slot| s.0.is_some_and(|p| p.caller == caller) && s.2 == token;
        self.slots.iter_mut().filter(theirs).for_each(|s| s.0 = None);
    }
}
