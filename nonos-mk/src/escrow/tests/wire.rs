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

//! The agent reads exactly what settle's encoder sends, and remembers a
//! proof of work for OFFER_TTL and no longer.

use nonos_nox_config::OFFER_TTL;

use super::super::body::{Asked, BODY_LEN};
use super::super::seen::Seen;
use super::buyer::request;

#[test]
fn the_agent_reads_settles_body_exactly() {
    let req = request(1);
    let b = req.agent_body().expect("a request whose solution fits its u16");
    let a = Asked::parse(&b).unwrap();
    assert_eq!(b.len(), BODY_LEN);
    assert_eq!((a.chain_id, a.buyer_pk, &a.pow[..]), (1, req.buyer_pk, &req.solution[..]));
    assert_eq!(Asked::parse(&[b.as_slice(), &[0]].concat()), None);
    let mut long = b.clone();
    long[104] = 16;
    assert_eq!(Asked::parse(&long), None, "the old 16-byte solution length");
}

#[test]
fn a_proof_of_work_is_answered_once_within_offer_ttl() {
    let mut s = Seen::default();
    let (pow, pk) = ([7u8; 36], [0u8; 32]);
    assert!(s.take(&pow, &pk, 100) && !s.take(&pow, &pk, 100 + OFFER_TTL - 1));
    assert!(s.take(&pow, &[1; 32], 100), "another buyer key is another request");
    assert!(s.take(&pow, &pk, 100 + OFFER_TTL), "past OFFER_TTL it is forgotten");
}
