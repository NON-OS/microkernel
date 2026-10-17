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

//! Reading ORDER_NEW's body: exactly ORDER_NEW_LEN bytes, in order, so a
//! body without refundSalt is refused. Its Debug never shows refundSalt
//! (rule 5): with refundTo it would open refundCommit.

use core::fmt;

use crate::cursor::Cursor;
use crate::order_new::OrderNew;

impl OrderNew {
    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (chain_id, listing_id, release_id, bond_id) =
            (r.u64()?, r.fixed()?, r.fixed()?, r.fixed()?);
        let (payee, amount, chunk_root, content_len) =
            (r.fixed()?, r.u128()?, r.fixed()?, r.u64()?);
        let (cipher_root, key_commit, deadline, refund_to, refund_salt) =
            (r.fixed()?, r.fixed()?, r.u64()?, r.fixed()?, r.fixed()?);
        r.end(OrderNew {
            chain_id,
            listing_id,
            release_id,
            bond_id,
            payee,
            amount,
            chunk_root,
            content_len,
            cipher_root,
            key_commit,
            deadline,
            refund_to,
            refund_salt,
        })
    }
}

impl fmt::Debug for OrderNew {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OrderNew")
            .field("chain_id", &self.chain_id)
            .field("listing_id", &self.listing_id)
            .field("release_id", &self.release_id)
            .field("bond_id", &self.bond_id)
            .field("payee", &self.payee)
            .field("amount", &self.amount)
            .field("chunk_root", &self.chunk_root)
            .field("content_len", &self.content_len)
            .field("cipher_root", &self.cipher_root)
            .field("key_commit", &self.key_commit)
            .field("deadline", &self.deadline)
            .field("refund_to", &self.refund_to)
            .field("refund_salt", &"redacted")
            .finish()
    }
}
