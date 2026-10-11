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

//! The escrow agent's sealed answer (plan/22 step 1, v1.4.24): agent_pk[32]
//! | ChaCha20-Poly1305 over the Offer, keyed by keccak256(ANSWER_TAG |
//! X25519(agent_sk, buyer_pk) | agent_pk | buyer_pk), nonce zero, with
//! buyer_pk | agent_pk as associated data. The agent draws agent_sk for each
//! request, so no key seals twice. The meter alone holds buyer_sk.

use alloc::vec;
use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;
use nonos_seal::{open, seal, NONCE_LEN, TAG_LEN};

use crate::answer::Offer;
use crate::answer_types::{answer_key, AnswerWrong, X25519};

pub fn seal_answer<K: Keccak, X: X25519>(
    (k, x): (&K, &X),
    agent_sk: &[u8; 32],
    buyer_pk: &[u8; 32],
    offer: &Offer,
) -> Option<Vec<u8>> {
    let (agent_pk, shared) = (x.public(agent_sk), x.dh(agent_sk, buyer_pk));
    (shared != [0; 32]).then_some(())?;
    let key = answer_key(k, &shared, (&agent_pk, buyer_pk));
    let plain = offer.encode();
    let mut out = vec![0u8; 32 + plain.len() + TAG_LEN];
    out[..32].copy_from_slice(&agent_pk);
    let ad = [&buyer_pk[..], &agent_pk].concat();
    seal(&key, &[0; NONCE_LEN], &ad, &plain, &mut out[32..]).ok()?;
    Some(out)
}

pub fn open_answer<K: Keccak, X: X25519>(
    (k, x): (&K, &X),
    buyer_sk: &[u8; 32],
    sealed: &[u8],
) -> Result<Offer, AnswerWrong> {
    let (agent_pk, body) = sealed.split_first_chunk::<32>().ok_or(AnswerWrong::Short)?;
    (body.len() >= TAG_LEN).then_some(()).ok_or(AnswerWrong::Short)?;
    let (buyer_pk, shared) = (x.public(buyer_sk), x.dh(buyer_sk, agent_pk));
    (shared != [0; 32]).then_some(()).ok_or(AnswerWrong::LowOrder)?;
    let key = answer_key(k, &shared, (agent_pk, &buyer_pk));
    let mut plain = vec![0u8; body.len() - TAG_LEN];
    let ad = [&buyer_pk[..], agent_pk].concat();
    open(&key, &[0; NONCE_LEN], &ad, body, &mut plain).map_err(|_| AnswerWrong::Seal)?;
    Offer::parse(&plain).ok_or(AnswerWrong::Body)
}
