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

//! An order's ids: orderId = keccak256(abi.encode(ORDER_TAG,
//! chainId, escrow, terms)); refundCommit = keccak256(abi.encode(REFUND_TAG,
//! refundTo, refundSalt)).

use nonos_marketplace_abi::Keccak;
use nonos_nox_config::{ORDER_TAG, REFUND_TAG};

use crate::terms::{OrderTerms, TERMS_WORDS};

/// `tail` right-aligned in a word (callers pass at most 32 bytes).
pub(crate) fn word(tail: &[u8]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w.iter_mut().rev().zip(tail.iter().rev()).for_each(|(o, b)| *o = *b);
    w
}

pub fn order_id(k: &impl Keccak, chain_id: u64, escrow: &[u8; 20], t: &OrderTerms) -> [u8; 32] {
    let mut pre = [0u8; 32 * (3 + TERMS_WORDS)];
    pre[..32].copy_from_slice(&ORDER_TAG);
    pre[32..64].copy_from_slice(&word(&chain_id.to_be_bytes()));
    pre[64..96].copy_from_slice(&word(escrow));
    let terms = pre.chunks_exact_mut(32).skip(3);
    terms.zip(t.words().iter()).for_each(|(slot, w)| slot.copy_from_slice(w));
    k.keccak256(&pre)
}

pub fn refund_commit(k: &impl Keccak, refund_to: &[u8; 20], salt: &[u8; 32]) -> [u8; 32] {
    let mut pre = [0u8; 96];
    pre[..32].copy_from_slice(&REFUND_TAG);
    pre[32..64].copy_from_slice(&word(refund_to));
    pre[64..].copy_from_slice(salt);
    k.keccak256(&pre)
}
