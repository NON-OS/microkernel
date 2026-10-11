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

/*
 * Where money is known due at a once-only address (B7, v1.4.56): a refund
 * or dispute this wallet sent, or an order whose escrow owes a refund. Only these are ever read over the network, one at a
 * time, each at most once per REREAD_MS, so the RPC host never sees the
 * handed list together. An entry goes once its money has moved on. Pure.
 */

use alloc::vec::Vec;

pub use super::due_entry::{Entry, Why};

pub const MAX_DUE: usize = 32;
pub const REREAD_MS: u64 = 600_000;

#[derive(Default)]
pub struct Due {
    pub entries: Vec<Entry>,
}

impl Due {
    /// Note money due at `address`; once per address. The oldest goes when full.
    pub fn add(&mut self, address: [u8; 20], why: Why, now_ms: u64) {
        if self.entries.iter().any(|e| e.address == address) {
            return;
        }
        if self.entries.len() == MAX_DUE {
            self.entries.remove(0);
        }
        let e = Entry { address, why, since_ms: now_ms, read_ms: None, nox: None };
        self.entries.push(e);
    }

    /// The address to read now, if any: never read, or not for REREAD_MS.
    pub fn next(&self, now_ms: u64) -> Option<[u8; 20]> {
        let stale = |e: &&Entry| e.read_ms.is_none_or(|r| now_ms.saturating_sub(r) >= REREAD_MS);
        self.entries.iter().filter(stale).min_by_key(|e| e.read_ms).map(|e| e.address)
    }

    /// A read of `address`: NOX found, or None when it did not answer. An
    /// address that held NOX and now holds none has had it moved on.
    pub fn read(&mut self, address: [u8; 20], nox: Option<u128>, now_ms: u64) {
        let Some(e) = self.entries.iter_mut().find(|e| e.address == address) else { return };
        e.read_ms = Some(now_ms);
        let moved = e.nox.is_some_and(|n| n > 0) && nox == Some(0);
        e.nox = nox.or(e.nox);
        if moved {
            self.entries.retain(|e| e.address != address);
        }
    }
}
