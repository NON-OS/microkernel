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

//! Which sends are the publisher's, what each path's waiting ones hold
//! back, and a nonce the chain read unused after a send that may not have
//! gone. One account, one nonce sequence: next_nonce is both paths' rule.

use super::*;

impl Ledger {
    /// A publisher transaction signed, before it is broadcast: from then on
    /// it may land, and both paths count it.
    pub fn record_publisher(&mut self, sent: Sent) {
        self.record(sent);
        if !self.publisher.contains(&sent.hash) {
            self.publisher.push(sent.hash);
        }
        let list = &self.list;
        self.publisher.retain(|h| list.iter().any(|s| s.hash == *h));
    }

    /// Whether a send still waits for a block on this network and account:
    /// one of the publisher's (`publisher`), or one of a payment's.
    pub fn waiting_of(&self, chain: u64, from: &[u8; 20], publisher: bool) -> bool {
        self.of(chain, from)
            .filter(|s| s.fate == Fate::Waiting)
            .any(|s| self.publisher.contains(&s.hash) == publisher)
    }

    /// A send whose broadcast may not have gone, its nonce read unused on
    /// chain: freed as a dropped one's, so only the same nonce is offered
    /// again, and only through the Sign press.
    pub fn unused(&mut self, hash: &[u8; 32]) {
        if let Some(s) = self.list.iter_mut().find(|s| s.hash == *hash && s.fate == Fate::Waiting) {
            s.fate = Fate::Dropped;
        }
    }
}
