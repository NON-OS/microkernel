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

//! The prover's refusals and the proof fetch's statuses by name (I-B9,
//! plan/14): each its own sentence, OUTSIDE plainly apart from PROVER.

use crate::store::prove_said::*;

#[test]
fn each_prover_refusal_has_its_own_sentence() {
    let codes = [STALE_REGISTRY, POLICY, NOT_ENROLLED, PROVER, OUTSIDE];
    assert_eq!(codes, [-1101, -1102, -1103, -1104, -1105], "plan/14's numbers");
    let said: Vec<_> = codes.iter().map(|c| prove_refused(*c).unwrap()).collect();
    for (i, a) in said.iter().enumerate() {
        assert!(said[i + 1..].iter().all(|b| a != b), "refusal {i} is its own");
    }
    assert_eq!(
        prove_refused(OUTSIDE),
        Some(&b"A call outside the set succeeded: nothing true to prove"[..])
    );
    assert!(!prove_refused(OUTSIDE).unwrap().windows(6).any(|w| w == b"prover"));
    assert!(prove_refused(PROVER).unwrap().starts_with(b"The prover failed"));
    assert_eq!(prove_refused(-1106), None);
    assert_eq!(prove_refused(-16), None, "EBUSY is not the prover's refusal");
}

#[test]
fn each_fetch_status_is_named_and_pending_is_not_a_failure() {
    let codes = [NO_RELAY, PENDING, ROUTE, ANCHOR, NOT_CONTAINER, RELAY];
    assert_eq!(codes, [-1201, -1202, -1203, -1204, -1205, -1206], "plan/14's numbers");
    let said: Vec<_> = codes.iter().map(|c| fetch_said(*c).unwrap()).collect();
    for (i, a) in said.iter().enumerate() {
        assert!(said[i + 1..].iter().all(|b| a != b), "status {i} is its own");
    }
    assert_eq!(fetch_said(PENDING), Some(&b"The proof is still downloading"[..]));
    assert!(fetch_said(ROUTE).unwrap().windows(3).any(|w| w == b"Nym"));
    assert_eq!(fetch_said(OUTSIDE), None, "the prover's codes are not the fetch's");
}
