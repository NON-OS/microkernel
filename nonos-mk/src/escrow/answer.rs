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

//! One answer (plan/22 step 1): a body that parses, a chain the config knows,
//! this release's listing_key and release_key, an Equi-X solution over
//! listing_key | buyer_pk that verifies and was not answered before; then a
//! fresh K and a fresh agent key, the offer, sealed to buyer_pk. K is
//! returned for the seller's key file and is revealed only by deliver().

use nonos_equix::{verify, Solution};
use nonos_nox_chunks::{equix_challenge, seal_answer};
use nonos_nox_config::Chain;

use super::body::Asked;
use super::offer::offer;
use super::seen::Seen;
use super::x::Dalek;
use crate::hosts::Host;

/// The release this agent sells, its keys and its bytes.
pub struct Served {
    pub listing_key: [u8; 32],
    pub release_key: [u8; 32],
    pub content: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    Body,
    Chain,
    NotServed,
    Solution,
    Seen,
    /// buyer_pk makes no shared secret, or the content is empty.
    Seal,
}

pub fn answer(
    s: &Served,
    seen: &mut Seen,
    body: &[u8],
    draw: &mut impl FnMut() -> [u8; 32],
) -> Result<(Vec<u8>, [u8; 32]), Refusal> {
    let a = Asked::parse(body).ok_or(Refusal::Body)?;
    Chain::from_id(a.chain_id).ok_or(Refusal::Chain)?;
    if (a.listing_key, a.release_key) != (s.listing_key, s.release_key) {
        return Err(Refusal::NotServed);
    }
    let challenge = equix_challenge(&a.listing_key, &a.buyer_pk);
    verify(&challenge, &Solution::from_bytes(&a.solution)).map_err(|_| Refusal::Solution)?;
    if !seen.take(&a.solution, &a.buyer_pk) {
        return Err(Refusal::Seen);
    }
    let (k, agent_sk) = (draw(), draw());
    let o = offer(&s.content, &k).ok_or(Refusal::Seal)?;
    let sealed = seal_answer((&Host, &Dalek), &agent_sk, &a.buyer_pk, &o).ok_or(Refusal::Seal)?;
    Ok((sealed, k))
}
