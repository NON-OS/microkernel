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

//! ORDER_OFFER's pages (plan/29): settle answers FETCH 1 and each PAGE with
//! a Page of the sealed answer. Its total must be agent_pk, the offer for
//! this content_len and a tag, exactly; each page must follow the last. Then
//! the meter opens it under buyer_sk for this release, runs check_offer, and
//! keeps the offer before answering offerId, cipherRoot and keyCommit.

use alloc::boxed::Box;

use nonos_marketplace_abi::Keccak;
use nonos_nox_chunks::{check_offer, offer_len, open_answer, X25519};
use nonos_payment_proto::OfferMade;
use nonos_settle_proto::{Fetch, Page};

use super::ask::{Ask, Next, Step};
use super::step::Flow;
use super::work::Work;
use crate::error::MeterError;
use crate::escrow::KeptOffer;

impl Flow {
    pub fn on_page<K: Keccak, X: X25519>(&mut self, page: &Page, (k, x): (&K, &X)) -> Step {
        let Work::Offer(w) = &mut self.work else { return Err(MeterError::Mismatch) };
        let total = offer_len(w.content_len).ok_or(MeterError::Mismatch)? + 32 + 16;
        if page.total as usize != total || page.offset as usize != w.fetched.len() {
            return Err(MeterError::Mismatch);
        }
        w.fetched.extend_from_slice(&page.bytes);
        if w.fetched.len() < total && !page.bytes.is_empty() {
            let more = Fetch::Page { release_hash: w.buyer_pk, offset: w.fetched.len() as u32 };
            return Ok(Next::Ask(Ask::Fetch(more.encode().ok_or(MeterError::Mismatch)?)));
        }
        (w.fetched.len() == total).then_some(()).ok_or(MeterError::Mismatch)?;
        let keys = (&w.want.listing_key, &w.want.release_key);
        let offer = open_answer((k, x), &w.buyer_sk, (keys, w.content_len), &w.fetched)
            .map_err(|_| MeterError::Untrusted)?;
        let (h, c) = (&offer.hashes, &offer.ciphertext);
        check_offer(k, &w.chunk_root, w.content_len, h, c, &offer.cipher_root)
            .map_err(|_| MeterError::Untrusted)?;
        let kept = KeptOffer {
            chain_id: w.want.chain_id,
            listing_key: w.want.listing_key,
            release_key: w.want.release_key,
            chunk_root: w.chunk_root,
            content_len: w.content_len,
            buyer_sk: w.buyer_sk,
            made_at: w.made_at,
            offer,
        };
        let (cipher_root, key_commit) = (kept.offer.cipher_root, kept.offer.key_commit);
        let made = OfferMade { offer_id: kept.offer_id(k), cipher_root, key_commit };
        Ok(Next::KeepOffer(Box::new(kept), made.encode()))
    }
}
