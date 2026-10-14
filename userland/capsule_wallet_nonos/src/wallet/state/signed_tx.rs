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
//! The signed transaction: its bytes, hash and nonce, what it costs and
//! moves, and the fees its review was offered.

use alloc::vec::Vec;

pub struct SignedTx {
    pub hash: [u8; 32],
    /// The signed transaction's nonce, the most ETH it can take, and the
    /// token it moves, for the ledger once it goes.
    pub nonce: u64,
    pub cost: u128,
    pub token: Option<(u8, u128)>,
    /// The tip and the cap per gas the last review read from the network's
    /// recent blocks (`send::fees`), or None when it did not come.
    pub offered_fees: Option<(u128, u128)>,
    pub len: u32,
    pub raw: Vec<u8>,
    pub ready: bool,
    pub kind: &'static [u8],
}

impl SignedTx {
    pub fn new() -> Self {
        Self {
            hash: [0; 32],
            nonce: 0,
            cost: 0,
            token: None,
            offered_fees: None,
            len: 0,
            raw: Vec::new(),
            ready: false,
            kind: b"none",
        }
    }
}
