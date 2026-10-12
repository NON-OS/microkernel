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

//! ORDER_DELIVERED (plan/21, plan/29): K only if keccak256(abi.encode(K)) is
//! the order's keyCommit, then open_delivery over the order's own offer.
//! Every chunk checking gives the plaintext, kept for op 18; a bad chunk gives
//! dispute()'s calldata for the wallet's Dispute intent, chunk i's proofs
//! from the offer the meter checked before funding.

use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;
use nonos_nox_chunks::{dispute, dispute_call, open_delivery, Delivered, OrderTerms};

use super::record::Order;
use super::take::order_takes;
use crate::error::MeterError;
use crate::escrow::KeptOffer;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Opened {
    Plain(Vec<u8>),
    /// The first bad chunk, and dispute(T, i, ...)'s calldata.
    Bad(u64, Vec<u8>),
}

pub fn deliver(
    k: &impl Keccak,
    o: &Order,
    kept: &KeptOffer,
    key: &[u8; 32],
) -> Result<Opened, MeterError> {
    if !order_takes(o, kept) {
        return Err(MeterError::Mismatch);
    }
    let (h, c) = (&kept.offer.hashes, &kept.offer.ciphertext);
    match open_delivery(k, key, &kept.offer.key_commit, h, c, kept.content_len) {
        Ok(plain) => Ok(Opened::Plain(plain)),
        Err(Delivered::WrongKey) => Err(MeterError::Untrusted),
        Err(Delivered::NotTheOffer) => Err(MeterError::Mismatch),
        Err(Delivered::BadChunk(i)) => {
            let terms = OrderTerms::from_words(&o.terms).ok_or(MeterError::Corrupt)?;
            let d = dispute(k, i, h, c).ok_or(MeterError::Corrupt)?;
            Ok(Opened::Bad(i, dispute_call(&terms, &d).ok_or(MeterError::Corrupt)?))
        }
    }
}
