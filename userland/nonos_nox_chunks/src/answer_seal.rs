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

//! The escrow agent's sealed answer (plan/29): agent_pk[32] |
//! ChaCha20-Poly1305 over the Offer, keyed by keccak256(ANSWER_TAG | X25519
//! shared | agent_pk | buyer_pk), nonce zero, with listing_key | release_key
//! as associated data, so an answer for another release never opens. The
//! buyer key is fresh per offer and the agent's per answer: no key seals twice.

use alloc::vec;
use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;
use nonos_seal::{open, seal, NONCE_LEN, TAG_LEN};

use crate::answer::Offer;
use crate::answer_types::{answer_key, AnswerWrong, X25519};

/// listing_key and release_key, the request's two.
pub type Release<'a> = (&'a [u8; 32], &'a [u8; 32]);

pub fn seal_answer<K: Keccak, X: X25519>(
    (k, x): (&K, &X),
    agent_sk: &[u8; 32],
    buyer_pk: &[u8; 32],
    (listing, release): Release<'_>,
    offer: &Offer,
) -> Option<Vec<u8>> {
    let (agent_pk, shared) = (x.public(agent_sk), x.dh(agent_sk, buyer_pk));
    (!all_zero(&shared)).then_some(())?;
    let key = answer_key(k, &shared, (&agent_pk, buyer_pk));
    let plain = offer.encode();
    let mut sealed = vec![0u8; plain.len().checked_add(TAG_LEN)?];
    let ad = [&listing[..], release].concat();
    seal(&key, &[0; NONCE_LEN], &ad, &plain, &mut sealed).ok()?;
    Some([&agent_pk[..], &sealed].concat())
}

/// The offer for `content_len`, opened under buyer_sk for this release.
pub fn open_answer<K: Keccak, X: X25519>(
    (k, x): (&K, &X),
    buyer_sk: &[u8; 32],
    ((listing, release), content_len): (Release<'_>, u64),
    sealed: &[u8],
) -> Result<Offer, AnswerWrong> {
    let (agent_pk, body) = sealed.split_first_chunk::<32>().ok_or(AnswerWrong::Short)?;
    let plain_len = body.len().checked_sub(TAG_LEN).ok_or(AnswerWrong::Short)?;
    let (buyer_pk, shared) = (x.public(buyer_sk), x.dh(buyer_sk, agent_pk));
    (!all_zero(&shared)).then_some(()).ok_or(AnswerWrong::LowOrder)?;
    let key = answer_key(k, &shared, (agent_pk, &buyer_pk));
    let mut plain = vec![0u8; plain_len];
    let ad = [&listing[..], release].concat();
    open(&key, &[0; NONCE_LEN], &ad, body, &mut plain).map_err(|_| AnswerWrong::Seal)?;
    Offer::parse(&plain, content_len).ok_or(AnswerWrong::Body)
}

/// A low-order point's shared secret is all zero. Every byte is read, so
/// how long the check takes says nothing about the secret.
fn all_zero(shared: &[u8; 32]) -> bool {
    shared.iter().fold(0u8, |acc, b| acc | b) == 0
}
