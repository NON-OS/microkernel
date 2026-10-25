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
 * Whether the order the meter made is the one the person confirmed: every
 * term the wallet sent, and the two the meter fills from the config, the
 * chain's NOX and DISPUTE_WINDOW. The salt is its own; refundCommit,
 * over the wallet's own refundTo and refundSalt, deposit_check recomputes.
 */

use nonos_nox_config::{address, Contract, DISPUTE_WINDOW};
use nonos_payment_proto::OrderNew;

/// The words as the escrow lays them (OrderTerms::words): ids whole,
/// numbers and addresses right-aligned big-endian.
pub(super) fn confirms(n: &OrderNew, w: &[[u8; 32]; 14]) -> bool {
    let word = |b: &[u8]| {
        let mut x = [0u8; 32];
        x[32 - b.len()..].copy_from_slice(b);
        x
    };
    let Ok(nox) = address(n.chain_id, Contract::NoxToken) else { return false };
    (w[0], w[1], w[2]) == (n.listing_id, n.release_id, n.bond_id)
        && (w[3], w[4]) == (word(&n.payee), word(&nox))
        && w[7] == word(&DISPUTE_WINDOW.to_be_bytes())
        && (w[5], w[6]) == (word(&n.amount.to_be_bytes()), word(&n.deadline.to_be_bytes()))
        && (w[8], w[9]) == (n.chunk_root, word(&n.content_len.to_be_bytes()))
        && (w[10], w[11]) == (n.cipher_root, n.key_commit)
}
