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

//! Which kept offer an order takes (plan/29): the same chain, listing,
//! release, chunk_root, content_len, cipherRoot and keyCommit, both when
//! ORDER_NEW asks (its body) and when the order is kept (its terms words).

use nonos_payment_proto::OrderNew;

use super::record::Order;
use crate::escrow::KeptOffer;

/// The kept offer ORDER_NEW names, with its slot, or None (ENOENT).
pub fn offer_for<'a>(
    n: &OrderNew,
    kept: &'a [(usize, KeptOffer)],
) -> Option<&'a (usize, KeptOffer)> {
    kept.iter().find(|(_, k)| {
        (k.chain_id, k.listing_key, k.release_key) == (n.chain_id, n.listing_id, n.release_id)
            && (k.chunk_root, k.content_len) == (n.chunk_root, n.content_len)
            && (k.offer.cipher_root, k.offer.key_commit) == (n.cipher_root, n.key_commit)
    })
}

/// Whether `k` is the offer the order's terms commit to.
pub fn order_takes(o: &Order, k: &KeptOffer) -> bool {
    let mut len = [0u8; 32];
    len[24..].copy_from_slice(&k.content_len.to_be_bytes());
    let w = &o.terms;
    o.chain_id == k.chain_id
        && (w[0], w[1], w[8], w[9]) == (k.listing_key, k.release_key, k.chunk_root, len)
        && (w[10], w[11]) == (k.offer.cipher_root, k.offer.key_commit)
}
