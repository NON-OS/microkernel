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

//! The bond cache's refresh, one bond at a time: READs 30 (bondOf), 31
//! (isActive), 33 (MIN_BOND) as ASYNC steps, one in flight. Pure.

use alloc::vec::Vec;

use super::cache::{BondCache, BondKey};
use super::late::row_of;

pub const STEP_MS: u64 = 150_000;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ask {
    pub chain: u64,
    pub view: u16,
    pub args: Vec<[u8; 32]>,
}

pub(super) struct Step {
    pub key: BondKey,
    pub(super) got: Vec<Vec<[u8; 32]>>,
    pub(super) sent_ms: u64,
}

impl BondCache {
    /// The queue head's first read, when nothing is in flight.
    pub fn next(&mut self, now_ms: u64) -> Option<Ask> {
        if self.step.is_some() || self.queue.is_empty() {
            return None;
        }
        let key = self.queue.remove(0);
        self.step = Some(Step { key, got: Vec::new(), sent_ms: now_ms });
        self.ask()
    }

    fn ask(&self) -> Option<Ask> {
        let s = self.step.as_ref()?;
        let (chain, id) = s.key;
        let view = *[30, 31, 33].get(s.got.len())?;
        let args = if view == 33 { Vec::new() } else { Vec::from([id]) };
        Some(Ask { chain, view, args })
    }

    /// The read in flight answered with words, or failed (None); the next ask.
    pub fn on_answer(&mut self, words: Option<Vec<[u8; 32]>>, now_ms: u64) -> Option<Ask> {
        let mut s = self.step.take()?;
        if let Some(w) = words {
            s.got.push(w);
            if s.got.len() < 3 {
                s.sent_ms = now_ms;
                self.step = Some(s);
                return self.ask();
            }
            if let Some(row) = row_of(&s, now_ms / 1000) {
                self.put(row);
            }
        }
        self.next(now_ms)
    }
}
