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

//! The agent answers each proof of work once within OFFER_TTL (plan/29).
//! At most SEEN_MAX are held; past that the oldest goes first, so a replay
//! of it costs one more answer, never a wrong one.

use std::collections::{HashMap, VecDeque};

use nonos_nox_chunks::POW_FIELD_LEN;
use nonos_nox_config::OFFER_TTL;

pub const SEEN_MAX: usize = 1 << 16;

type Key = ([u8; POW_FIELD_LEN], [u8; 32]);

#[derive(Default)]
pub struct Seen {
    at: HashMap<Key, u64>,
    order: VecDeque<(Key, u64)>,
}

impl Seen {
    /// True unless this (proof of work, buyer_pk) was answered within OFFER_TTL of `now`.
    pub fn take(&mut self, pow: &[u8; POW_FIELD_LEN], buyer_pk: &[u8; 32], now: u64) -> bool {
        while let Some((k, t)) = self.order.front().copied() {
            if now.saturating_sub(t) < OFFER_TTL && self.order.len() < SEEN_MAX {
                break;
            }
            self.order.pop_front();
            if self.at.get(&k) == Some(&t) {
                self.at.remove(&k);
            }
        }
        let k = (*pow, *buyer_pk);
        if self.at.contains_key(&k) {
            return false;
        }
        self.at.insert(k, now);
        self.order.push_back((k, now));
        true
    }
}
