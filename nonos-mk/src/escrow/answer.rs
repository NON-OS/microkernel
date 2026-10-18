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

//! One answer (plan/29): a body that parses, a chain the config knows, this
//! release's listing_key and release_key, a proof of work pow_check takes at
//! ESCROW_POW_EFFORT and not answered within OFFER_TTL; then a fresh K and a
//! fresh agent key, the offer, sealed to buyer_pk for this release. K and
//! the offer's keyCommit and cipherRoot go to the seller's key file.

use nonos_nox_chunks::{pow_check, seal_answer};
use nonos_nox_config::{Chain, ESCROW_POW_EFFORT};

use super::answer_types::{Answered, Refusal, Served};
use super::body::Asked;
use super::offer::offer;
use super::seen::Seen;
use super::x::Dalek;
use crate::hosts::Host;

pub fn answer(
    s: &Served,
    seen: &mut Seen,
    body: &[u8],
    now: u64,
    draw: &mut impl FnMut() -> [u8; 32],
) -> Result<Answered, Refusal> {
    let a = Asked::parse(body).ok_or(Refusal::Body)?;
    Chain::from_id(a.chain_id).ok_or(Refusal::Chain)?;
    if (a.listing_key, a.release_key) != (s.listing_key, s.release_key) {
        return Err(Refusal::NotServed);
    }
    if !pow_check(&a.listing_key, &a.buyer_pk, &a.pow, ESCROW_POW_EFFORT) {
        return Err(Refusal::Solution);
    }
    if !seen.take(&a.pow, &a.buyer_pk, now) {
        return Err(Refusal::Seen);
    }
    let (k, agent_sk) = (draw(), draw());
    let o = offer(&s.content, &k).ok_or(Refusal::Seal)?;
    let keys = (&s.listing_key, &s.release_key);
    let sealed =
        seal_answer((&Host, &Dalek), &agent_sk, &a.buyer_pk, keys, &o).ok_or(Refusal::Seal)?;
    let (key_commit, cipher_root, buyer_pk) = (o.key_commit, o.cipher_root, a.buyer_pk);
    Ok(Answered { sealed, k, key_commit, cipher_root, buyer_pk })
}
