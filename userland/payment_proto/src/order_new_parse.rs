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

//! Reading ORDER_NEW's body: exactly ORDER_NEW_LEN bytes, in order.

use crate::cursor::Cursor;
use crate::order_new::OrderNew;

impl OrderNew {
    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (chain_id, listing_id, release_id, bond_id) =
            (r.u64()?, r.fixed()?, r.fixed()?, r.fixed()?);
        let (payee, amount, chunk_root, content_len) =
            (r.fixed()?, r.u128()?, r.fixed()?, r.u64()?);
        let (cipher_root, key_commit, deadline, refund_to) =
            (r.fixed()?, r.fixed()?, r.u64()?, r.fixed()?);
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
        })
    }
}
