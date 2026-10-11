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

//! Escrow's fair delivery: content cut into 1024-byte chunks, the
//! plain and cipher Merkle trees over them, the per-order keystream, and the
//! order and refund commitments. Pure and no_std; keccak comes from the
//! caller, as marketplace_abi takes it.

#![no_std]

extern crate alloc;

mod answer;
mod answer_seal;
mod answer_types;
mod buyer;
mod buyer_types;
mod calls;
mod chunk;
mod cipher;
mod delivery;
mod deposit;
mod dispute;
mod dispute_call;
mod leaf;
mod offer;
mod order;
mod pin;
mod proof;
mod roots;
mod seller;
mod seller_types;
mod terms;
mod tree;

pub use answer::Offer;
pub use answer_seal::{open_answer, seal_answer};
pub use answer_types::{equix_challenge, AnswerWrong, ANSWER_TAG, X25519};
pub use buyer::buyer_terms;
pub use buyer_types::{Buyer, OrderWrong, Sale};
pub use calls::{fund, fund_from, refund, release, DISPUTE, FUND, FUND_FROM, REFUND, RELEASE};
pub use chunk::{chunk, chunk_count, CHUNK};
pub use cipher::{key_commit, keystream_word, xor_chunk};
pub use delivery::{open_delivery, Delivered};
pub use deposit::{clone_initcode_hash, create2, deposit_address};
pub use dispute::{dispute, Dispute};
pub use dispute_call::dispute_call;
pub use leaf::{leaf, CIPHER_TREE, PLAIN_TREE};
pub use offer::{check_offer, cipher_hashes, OfferWrong};
pub use order::{order_id, refund_commit};
pub use pin::{EscrowPin, PIN_LEN};
pub use proof::{proof, verify};
pub use roots::{chunk_root, cipher_root, plain_hashes, root_of_hashes};
pub use seller::{may_send, reveal_ok};
pub use seller_types::{Mine, TermsWrong};
pub use terms::{OrderTerms, TERMS_WORDS};
pub use tree::root;
