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


//! Descriptors already fetched and checked, so the next stream to a service
//! skips the HSDirs (hs_cache.c, client side).
//!
//! Keyed by blinded key and time period, so a period change can never serve
//! last period's descriptor. An entry lives for the descriptor's own
//! lifetime from when it was fetched, and a newer revision replaces an older
//! one but never the reverse. Only descriptors that decoded and checked are
//! stored; a failed lookup leaves nothing. It is held in memory only: no
//! entry is written to storage or named on the log.

extern crate alloc;

use alloc::vec::Vec;

use super::desc::IntroPoint;
use super::pow::PowParams;

/// Which service, under which key, in which period.
#[derive(Clone, Copy)]
pub struct CacheKey {
    pub identity: [u8; 32],
    pub blinded: [u8; 32],
    pub period: u64,
}

/// Services held at once. One more evicts the entry closest to expiry.
pub const CACHE_MAX: usize = 16;

struct Entry {
    identity: [u8; 32],
    blinded: [u8; 32],
    period: u64,
    revision: u64,
    expires: u64,
    intro_points: Vec<IntroPoint>,
    pow: Option<PowParams>,
}

#[derive(Default)]
pub struct DescCache {
    entries: Vec<Entry>,
}

impl DescCache {
    /// Keep a checked descriptor fetched at `now`, with the puzzle it asks
    /// for. Refused, and `false`, when an entry for the same key and period
    /// holds a newer revision.
    pub fn insert(
        &mut self,
        key: CacheKey,
        revision: u64,
        lifetime: u64,
        now: u64,
        intro_points: Vec<IntroPoint>,
        pow: Option<PowParams>,
    ) -> bool {
        let CacheKey { identity, blinded, period } = key;
        if let Some(at) = self.position(&blinded, period) {
            if self.entries[at].revision > revision {
                return false;
            }
            self.entries.remove(at);
        }
        if self.entries.len() >= CACHE_MAX {
            if let Some(soonest) = (0..self.entries.len()).min_by_key(|i| self.entries[*i].expires) {
                self.entries.remove(soonest);
            }
        }
        let expires = now.saturating_add(lifetime);
        self.entries.push(Entry { identity, blinded, period, revision, expires, intro_points, pow });
        true
    }

    /// The introduction points and puzzle cached for this key and period,
    /// while the descriptor's lifetime lasts. A puzzle seed past its expiry
    /// makes the entry a miss, as hs_client.c refetches then: the service has
    /// moved to a new seed, and a solution for the old one stops counting.
    pub fn get(&self, blinded: &[u8; 32], period: u64, now: u64) -> Option<(&[IntroPoint], Option<PowParams>)> {
        let entry = &self.entries[self.position(blinded, period)?];
        let seed_stale = entry.pow.is_some_and(|p| p.expires < now);
        (now < entry.expires && !seed_stale).then_some((entry.intro_points.as_slice(), entry.pow))
    }

    /// Forget one service's descriptor: its introduction points all failed,
    /// so the service has likely moved on and the HSDirs are asked again.
    pub fn forget(&mut self, blinded: &[u8; 32], period: u64) {
        self.entries.retain(|e| !(e.blinded == *blinded && e.period == period));
    }

    /// Forget every descriptor of the service with this identity: its
    /// client authorization key changed, so it is opened again.
    pub fn forget_service(&mut self, identity: &[u8; 32]) {
        self.entries.retain(|e| e.identity != *identity);
    }

    /// Drop every entry of another period, and every expired one.
    pub fn prune(&mut self, period: u64, now: u64) {
        self.entries.retain(|e| e.period == period && now < e.expires);
    }

    fn position(&self, blinded: &[u8; 32], period: u64) -> Option<usize> {
        self.entries.iter().position(|e| e.blinded == *blinded && e.period == period)
    }
}
