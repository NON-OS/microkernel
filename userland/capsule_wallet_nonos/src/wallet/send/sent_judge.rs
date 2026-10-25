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
//! What the payments still waiting hold back, and judging each reading.

use super::*;

impl Ledger {
    /// The hashes whose fate the chain may still change, oldest first.
    pub fn open(&self, chain: u64, from: &[u8; 20]) -> Vec<[u8; 32]> {
        self.of(chain, from).filter(|s| s.fate.open()).map(|s| s.hash).collect()
    }

    /// What waiting payments may still take, in ETH and in `asset`: held
    /// back from the balance, since the balance read does not count them.
    pub fn held(&self, chain: u64, from: &[u8; 20], asset: u8) -> (u128, u128) {
        self.of(chain, from).filter(|s| s.fate == Fate::Waiting).fold((0, 0), |(e, t), s| {
            let token = match s.token {
                Some((a, n)) if a == asset => n,
                _ => 0,
            };
            (e.saturating_add(s.eth_cost), t.saturating_add(token))
        })
    }

    /// Whether a broadcast that broke off, on this network and account, may
    /// still land: one sent under `hold_ms` ago that no reading has settled.
    /// A new transaction waits for it, so none is built behind a nonce that
    /// may never be used.
    pub fn unknown_waiting(&self, chain: u64, from: &[u8; 20], now: i64, hold_ms: i64) -> bool {
        self.of(chain, from)
            .any(|s| s.unknown && s.fate == Fate::Waiting && now.saturating_sub(s.at_ms) < hold_ms)
    }

    pub fn get(&self, hash: &[u8; 32]) -> Option<&Sent> {
        self.list.iter().find(|s| s.hash == *hash)
    }

    /// Take a reading for `hash` at `now`. Returns the fate when it changed.
    pub fn judge(&mut self, hash: &[u8; 32], r: Reading, now: i64) -> Option<Fate> {
        let s = self.list.iter_mut().find(|s| s.hash == *hash)?;
        if !s.fate.open() {
            return None;
        }
        let used = r.receipt.is_none() && r.latest > s.nonce;
        s.misses = if used { s.misses.saturating_add(1) } else { 0 };
        let next = fate(s, r, now);
        (next != s.fate).then(|| {
            s.fate = next;
            next
        })
    }
}
