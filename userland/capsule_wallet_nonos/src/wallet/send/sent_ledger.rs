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
//! Recording a payment, and the nonce the next one takes.

use super::*;

impl Ledger {
    pub fn record(&mut self, sent: Sent) {
        self.list.retain(|s| s.hash != sent.hash);
        self.list.push(sent);
        let settled = self.list.iter().filter(|s| !s.fate.open()).count();
        let mut drop = settled.saturating_sub(SETTLED_KEPT);
        self.list.retain(|s| {
            if drop > 0 && !s.fate.open() {
                drop -= 1;
                return false;
            }
            true
        });
    }

    pub(super) fn of<'a>(
        &'a self,
        chain: u64,
        from: &'a [u8; 20],
    ) -> impl Iterator<Item = &'a Sent> + 'a {
        self.list.iter().filter(move |s| s.chain == chain && s.from == *from)
    }

    /// The nonce the next transaction takes: the node's pending count, but
    /// never one this wallet already signed and sent, unless that one was
    /// dropped and its nonce is free again. None when there is none: one past
    /// u64::MAX would repeat a signed nonce if it saturated, and no
    /// transaction may carry u64::MAX itself (EIP-2681).
    pub fn next_nonce(&self, chain: u64, from: &[u8; 20], pending: u64) -> Option<u64> {
        self.of(chain, from)
            .filter(|s| s.fate != Fate::Dropped)
            .try_fold(pending, |most, s| Some(most.max(s.nonce.checked_add(1)?)))
            .and_then(usable)
    }
}

/// A nonce a transaction may carry, or None: none may carry u64::MAX
/// (EIP-2681), so a node that reports it is not believed.
pub fn usable(nonce: u64) -> Option<u64> {
    (nonce < u64::MAX).then_some(nonce)
}
