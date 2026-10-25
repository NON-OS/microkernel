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

//! A server silent to one caller: only late calls, no answer in time, for the
//! kernel's default wait or longer, is starved however short each call's own
//! deadline was. Kept per caller and endpoint in a fixed table, with no lock
//! and no allocation here. A full table evicts the pair silent longest and
//! reports it at once, so an eviction makes a report earlier, never drops one.

pub(crate) const SLOTS: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Pair {
    pub(crate) caller: u32,
    pub(crate) port: u32,
}

/// A pair and when its silence began; `None` is a free slot.
type Slot = (Option<Pair>, u64);

pub(crate) struct Silence {
    slots: [Slot; SLOTS],
    window_ms: u64,
}

impl Silence {
    pub(crate) const fn new(window_ms: u64) -> Self {
        Self { slots: [(None, 0); SLOTS], window_ms }
    }

    /// A call of `pair` begun at `start_ms` came back late at `now_ms`. The
    /// pairs to report unanswered now: this one, once silent the window, and
    /// one evicted to make room. A reported pair's silence starts again.
    pub(crate) fn late(&mut self, pair: Pair, start_ms: u64, now_ms: u64) -> [Option<Pair>; 2] {
        let mut out = [None, None];
        let at = match self.slots.iter().position(|s| s.0 == Some(pair)) {
            Some(at) => at,
            None => {
                let at = self.free_or_oldest();
                out[1] = self.slots[at].0;
                self.slots[at] = (Some(pair), start_ms);
                at
            }
        };
        if now_ms.saturating_sub(self.slots[at].1) >= self.window_ms {
            self.slots[at].1 = now_ms;
            out[0] = Some(pair);
        }
        out
    }

    /// `pair` was answered in time: it is not silent.
    pub(crate) fn answered(&mut self, pair: Pair) {
        self.slots.iter_mut().filter(|s| s.0 == Some(pair)).for_each(|s| s.0 = None);
    }

    fn free_or_oldest(&self) -> usize {
        let free = self.slots.iter().position(|s| s.0.is_none());
        let oldest = || (0..SLOTS).min_by_key(|&i| self.slots[i].1).unwrap_or(0);
        free.unwrap_or_else(oldest)
    }
}
