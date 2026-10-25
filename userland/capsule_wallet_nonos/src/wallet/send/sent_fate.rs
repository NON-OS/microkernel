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
//! What became of a payment, as one reading of the chain says.

use super::*;

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
