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

//! OrderTerms. Every field is a static word, so abi.encode(terms)
//! is its fourteen words in order, the tuple settle_proto's ORDER_TERMS_TUPLE
//! names.

pub const TERMS_WORDS: usize = 14;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OrderTerms {
    pub listing_id: [u8; 32],
    pub release_id: [u8; 32],
    pub bond_id: [u8; 32],
    pub payee: [u8; 20],
    pub token: [u8; 20],
    pub amount: u128,
    pub delivery_deadline: u64,
    pub dispute_window: u64,
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    pub cipher_root: [u8; 32],
    pub key_commit: [u8; 32],
    pub refund_commit: [u8; 32],
    pub salt: [u8; 32],
}

impl OrderTerms {
    pub fn words(&self) -> [[u8; 32]; TERMS_WORDS] {
        [
            self.listing_id,
            self.release_id,
            self.bond_id,
            crate::order::word(&self.payee),
            crate::order::word(&self.token),
            crate::order::word(&self.amount.to_be_bytes()),
            crate::order::word(&self.delivery_deadline.to_be_bytes()),
            crate::order::word(&self.dispute_window.to_be_bytes()),
            self.chunk_root,
            crate::order::word(&self.content_len.to_be_bytes()),
            self.cipher_root,
            self.key_commit,
            self.refund_commit,
            self.salt,
        ]
    }
}
