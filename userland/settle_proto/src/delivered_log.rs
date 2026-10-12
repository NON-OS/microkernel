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

//! A Delivered log: the escrow's word that an order's key is
//! public. orderId is its one indexed topic, the key its one data word;
//! anything else is no event. DELIVERED (op 12) keeps what it read of each
//! chain, at most MAX_KEYS pairs, then rescans per order.

use alloc::vec::Vec;

pub const DELIVERED: &str = "Delivered(bytes32,bytes32)";
/// keccak256(DELIVERED), pinned by a test.
pub const DELIVERED_TOPIC: [u8; 32] = [
    0x87, 0x20, 0x24, 0xbb, 0x43, 0x56, 0xec, 0xd1, 0x9d, 0xc9, 0xcb, 0xc3, 0xcf, 0x9c, 0x28, 0xc7,
    0x66, 0xda, 0x9e, 0xa9, 0xe1, 0x27, 0x6f, 0x6e, 0x46, 0x32, 0x16, 0xe8, 0x1a, 0xcd, 0x3c, 0x92,
];

/// (orderId, key) from a log's topics and data.
pub fn delivered(topics: &[[u8; 32]], data: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    let [t0, order_id] = topics else { return None };
    (*t0 == DELIVERED_TOPIC).then_some(())?;
    Some((*order_id, data.try_into().ok()?))
}

pub const MAX_KEYS: usize = 256;

/// One chain's Delivered logs read so far: the last block scanned, and
/// (orderId, key) for each, or `full` once they passed MAX_KEYS.
pub struct DeliveredKeys {
    pub chain_id: u64,
    pub scanned: u64,
    pub keys: Vec<([u8; 32], [u8; 32])>,
    pub full: bool,
}

impl DeliveredKeys {
    pub fn new(chain_id: u64) -> Self {
        DeliveredKeys { chain_id, scanned: 0, keys: Vec::new(), full: false }
    }

    pub fn find(&self, order_id: &[u8; 32]) -> Option<[u8; 32]> {
        self.keys.iter().find(|(o, _)| o == order_id).map(|(_, k)| *k)
    }

    /// Take in the pairs read after `scanned` up to `head`.
    pub fn absorb(&mut self, more: Vec<([u8; 32], [u8; 32])>, head: u64) {
        if self.full || self.keys.len().saturating_add(more.len()) > MAX_KEYS {
            (self.full, self.keys) = (true, Vec::new());
            return;
        }
        self.keys.extend(more);
        self.scanned = self.scanned.max(head);
    }
}
