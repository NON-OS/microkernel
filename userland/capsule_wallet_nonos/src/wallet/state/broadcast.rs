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
//! A transaction on its way: armed, sent or unknown, its receipt followed,
//! and the ledger of every one not yet settled.

use crate::wallet::act::Purpose;
use crate::wallet::send::sent::Ledger;

pub struct BroadcastState {
    /// A signed transaction waiting for its confirming second press. Sending is
    /// the one act in this window that cannot be undone, so it is armed rather
    /// than fired: the same shape the process manager uses before it ends a
    /// process, which is a far more recoverable thing to do.
    pub armed: bool,
    pub ready: bool,
    pub hash: [u8; 32],
    pub receipt_ready: bool,
    /// The broadcast broke once the transaction had begun to go, so whether
    /// the node took it is not known; it is never sent again, and its
    /// receipt is followed all the same.
    pub unknown: bool,
    pub receipt_ok: bool,
    /// What the receipt followed now is for, and when it went.
    pub purpose: Purpose,
    pub sent_at_ms: i64,
    /// Every transaction sent and not yet settled, per network and account,
    /// kept across a switch (`send::sent`).
    pub sent: Ledger,
}

impl BroadcastState {
    pub fn new() -> Self {
        Self {
            armed: false,
            ready: false,
            hash: [0; 32],
            receipt_ready: false,
            unknown: false,
            receipt_ok: false,
            purpose: Purpose::Pay,
            sent_at_ms: 0,
            sent: Ledger::default(),
        }
    }
}
