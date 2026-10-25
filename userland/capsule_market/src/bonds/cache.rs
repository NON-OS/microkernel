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

//! The market's bond cache: (chain, bondId) to the
//! BondReading and MIN_BOND that one refresh read, at most CACHE_MAX rows.
//! INSTALL_READY answers only from here; a bond missing, or due, is queued
//! for the market's own ASYNC READs. Pure.

use alloc::vec::Vec;

use nonos_marketplace_abi::BondReading;

use super::refresh::Step;

pub const CACHE_MAX: usize = 256;
pub const QUEUE_MAX: usize = 32;
/// A reading this old is read again when next asked for, before it is stale.
pub const REFRESH_AFTER: u64 = 1_800;

pub type BondKey = (u64, [u8; 32]);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Row {
    pub key: BondKey,
    pub reading: BondReading,
    pub min: u128,
}

#[derive(Default)]
pub struct BondCache {
    pub(super) rows: Vec<Row>,
    pub(super) queue: Vec<BondKey>,
    pub(super) step: Option<Step>,
}

impl BondCache {
    /// The cached reading and MIN_BOND, queuing a read when missing or due.
    pub fn reading(&mut self, key: BondKey, now: u64) -> Option<(BondReading, u128)> {
        let row = self.rows.iter().find(|r| r.key == key).copied();
        if row.is_none_or(|r| now.saturating_sub(r.reading.read_at) >= REFRESH_AFTER) {
            self.want(key);
        }
        row.map(|r| (r.reading, r.min))
    }

    fn want(&mut self, key: BondKey) {
        let busy = self.step.as_ref().is_some_and(|s| s.key == key);
        if !busy && !self.queue.contains(&key) && self.queue.len() < QUEUE_MAX {
            self.queue.push(key);
        }
    }
}
