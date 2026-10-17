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

//! ORDER_NEW (op 8, plan/21, v1.4.20), from the wallet alone; a job (op 15).
//!
//!   chain_id u64 | listingId[32] | releaseId[32] | bondId[32] | payee[20]
//!   | amount u128 | chunkRoot[32] | contentLen u64 | cipherRoot[32]
//!   | keyCommit[32] | deliveryDeadline u64 | refundTo[20] | refundSalt[32]
//!
//! refundSalt is the wallet's own draw: the meter commits refundTo under it
//! and never chooses it. Debug shows it redacted (order_new_parse.rs).
//! The job's result is orderId[32] | depositAddress[20] | the 14 term words.

use alloc::vec::Vec;

pub const ORDER_NEW_LEN: usize = 8 + 32 * 3 + 20 + 16 + 32 + 8 + 32 * 2 + 8 + 20 + 32;
pub const ORDER_MADE_LEN: usize = 32 + 20 + 32 * 14;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OrderNew {
    pub chain_id: u64,
    pub listing_id: [u8; 32],
    pub release_id: [u8; 32],
    pub bond_id: [u8; 32],
    pub payee: [u8; 20],
    pub amount: u128,
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    pub cipher_root: [u8; 32],
    pub key_commit: [u8; 32],
    pub deadline: u64,
    pub refund_to: [u8; 20],
    pub refund_salt: [u8; 32],
}

impl OrderNew {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(ORDER_NEW_LEN);
        out.extend_from_slice(&self.chain_id.to_le_bytes());
        [&self.listing_id[..], &self.release_id, &self.bond_id, &self.payee]
            .iter()
            .for_each(|p| out.extend_from_slice(p));
        out.extend_from_slice(&self.amount.to_le_bytes());
        out.extend_from_slice(&self.chunk_root);
        out.extend_from_slice(&self.content_len.to_le_bytes());
        [&self.cipher_root, &self.key_commit].iter().for_each(|p| out.extend_from_slice(*p));
        out.extend_from_slice(&self.deadline.to_le_bytes());
        out.extend_from_slice(&self.refund_to);
        out.extend_from_slice(&self.refund_salt);
        out
    }
}
