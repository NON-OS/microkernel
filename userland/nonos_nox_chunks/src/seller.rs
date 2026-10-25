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

//! What the seller's agent checks before it reveals K: it
//! delivers only for terms it rebuilt itself. chunkRoot and contentLen are
//! its release's, cipherRoot is its own ciphertext's under K, keyCommit is
//! K's, the token is NOX, the amount at least the price, the payee and
//! bond its own, the deadline inside [now + DELIVER_MARGIN, now +
//! MAX_DELIVERY] and the window exactly DISPUTE_WINDOW.

use nonos_marketplace_abi::Keccak;
use nonos_nox_config::{DELIVER_MARGIN, DISPUTE_WINDOW, MAX_DELIVERY};

use crate::cipher::key_commit;
use crate::seller_types::{Mine, TermsWrong};
use crate::terms::OrderTerms;

pub fn reveal_ok(
    k: &impl Keccak,
    t: &OrderTerms,
    m: &Mine,
    key: &[u8; 32],
    cipher_root: &[u8; 32],
    now: u64,
) -> Result<(), TermsWrong> {
    let release = (t.listing_id, t.release_id, t.chunk_root, t.content_len);
    if release != (m.listing_id, m.release_id, m.chunk_root, m.content_len) {
        return Err(TermsWrong::NotMyRelease);
    }
    if t.cipher_root != *cipher_root || t.key_commit != key_commit(k, key) {
        return Err(TermsWrong::NotMyCipher);
    }
    if t.token != m.nox || t.payee != m.payee || t.bond_id != m.bond_id || t.amount < m.price {
        return Err(TermsWrong::NotMyPrice);
    }
    if !may_send(t, now)
        || t.delivery_deadline > now.saturating_add(MAX_DELIVERY)
        || t.dispute_window != DISPUTE_WINDOW
    {
        return Err(TermsWrong::BadTimes);
    }
    Ok(())
}

/// Whether deliver may still be broadcast: never once now + DELIVER_MARGIN
/// passes the deadline.
pub fn may_send(t: &OrderTerms, now: u64) -> bool {
    now.saturating_add(DELIVER_MARGIN) <= t.delivery_deadline
}
