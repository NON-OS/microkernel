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

//! Every transaction this wallet sent, per network and account, until the
//! chain has settled it. It gives the next nonce a floor, so a second
//! payment can never take the first's nonce from a node that has not seen
//! the first yet. It holds back what a waiting payment may still spend. And
//! it says what became of each payment: in a block, confirmed under enough
//! blocks, replaced by another with its nonce, or dropped without ever
//! landing. It is kept across a network switch, so leaving and coming back
//! forgets nothing. Pure: wallet_proofs drives it with readings of its own.

use alloc::vec::Vec;

/// Blocks on top of a payment's own, its included, before it is said to be
/// confirmed: past any reorganisation either network has seen since the merge.
pub const CONFIRMATIONS: u64 = 12;
/// How long a payment no node holds, and no block has, is waited for before
/// it is said to be dropped and its nonce is free again.
pub const DROP_MS: i64 = 30 * 60 * 1000;
/// Settled entries kept, newest first, for the screens; waiting ones are
/// always kept.
const SETTLED_KEPT: usize = 16;
/// Readings in a row that show its nonce used and no receipt before it is
/// said to be replaced: one node's receipt index can lag its blocks.
pub const REPLACED_AFTER: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fate {
    /// Sent, not seen in a block yet.
    Waiting,
    /// In this block, which succeeded or reverted, not yet deep enough.
    InBlock { block: u64, ok: bool },
    /// Under `CONFIRMATIONS` blocks.
    Confirmed { ok: bool },
    /// Its nonce was used by another transaction that landed: this one never will.
    Replaced,
    /// No block has it and no node holds it: nothing of it was paid.
    Dropped,
}

impl Fate {
    /// Whether the chain may still change what became of it.
    pub fn open(self) -> bool {
        matches!(self, Fate::Waiting | Fate::InBlock { .. })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sent {
    pub chain: u64,
    pub from: [u8; 20],
    pub nonce: u64,
    pub hash: [u8; 32],
    pub at_ms: i64,
    /// The most ETH it can take: its value and its fee cap.
    pub eth_cost: u128,
    /// The token it moves, as the send screen numbers assets, and how much.
    pub token: Option<(u8, u128)>,
    pub fate: Fate,
    /// Readings in a row with its nonce used and no receipt.
    pub misses: u8,
    /// The broadcast broke off before the node said it took it.
    pub unknown: bool,
}

/// One reading of the chain for a sent transaction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reading {
    /// Its receipt: the block it is in and whether it succeeded, or None
    /// when the node has no receipt for it.
    pub receipt: Option<(u64, bool)>,
    /// The newest block.
    pub head: u64,
    /// The account's nonce in the newest block: every nonce under it is used.
    pub latest: u64,
    /// The account's nonce counting what the node holds waiting.
    pub pending: u64,
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Ledger {
    pub list: Vec<Sent>,
}

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

    fn of<'a>(&'a self, chain: u64, from: &'a [u8; 20]) -> impl Iterator<Item = &'a Sent> + 'a {
        self.list.iter().filter(move |s| s.chain == chain && s.from == *from)
    }

    /// The nonce the next transaction takes: the node's pending count, but
    /// never one this wallet already signed and sent, unless that one was
    /// dropped and its nonce is free again.
    pub fn next_nonce(&self, chain: u64, from: &[u8; 20], pending: u64) -> u64 {
        self.of(chain, from)
            .filter(|s| s.fate != Fate::Dropped)
            .map(|s| s.nonce.saturating_add(1))
            .fold(pending, u64::max)
    }

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

/// What one reading says became of `s`.
pub fn fate(s: &Sent, r: Reading, now: i64) -> Fate {
    match r.receipt {
        Some((block, ok)) if r.head >= block => {
            if r.head - block + 1 >= CONFIRMATIONS {
                Fate::Confirmed { ok }
            } else {
                Fate::InBlock { block, ok }
            }
        }
        /* A receipt for a block the node has not reached is not yet one. */
        Some(_) => s.fate,
        /* No receipt, and its nonce used in a block, reading after reading:
         * another took it. Until then, and after a block that held it was
         * reorganised away, it is waiting. */
        None if s.misses >= REPLACED_AFTER => Fate::Replaced,
        None if r.latest > s.nonce => Fate::Waiting,
        None if now.saturating_sub(s.at_ms) >= DROP_MS && r.pending <= s.nonce => Fate::Dropped,
        None => Fate::Waiting,
    }
}
