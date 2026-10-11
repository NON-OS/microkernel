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

//! Escrow's fair delivery (plan/22): content cut into 1024-byte chunks, the
//! plain and cipher Merkle trees over them, the per-order keystream, and the
//! order and refund commitments. Pure and no_std; keccak comes from the
//! caller, as marketplace_abi takes it.

#![no_std]

extern crate alloc;

mod chunk;
mod cipher;
mod delivery;
mod dispute;
mod leaf;
mod offer;
mod order;
mod proof;
mod roots;
mod seller;
mod seller_types;
mod terms;
mod tree;

pub use chunk::{chunk, chunk_count, CHUNK};
pub use cipher::{key_commit, keystream_word, xor_chunk};
pub use delivery::{open_delivery, Delivered};
pub use dispute::{dispute, Dispute};
pub use leaf::{leaf, CIPHER_TREE, PLAIN_TREE};
pub use offer::{check_offer, cipher_hashes, OfferWrong};
pub use order::{order_id, refund_commit};
pub use proof::{proof, verify};
pub use roots::{chunk_root, cipher_root, plain_hashes, root_of_hashes};
pub use seller::{may_send, reveal_ok};
pub use seller_types::{Mine, TermsWrong};
pub use terms::{OrderTerms, TERMS_WORDS};
pub use tree::root;
