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

//! FETCH 1 ESCROW's wire: the request round trips
//! exactly, refuses every cut and trailing byte and each length past its
//! limit, and the agent is sent the request without its url.

use nonos_settle_proto::*;

fn ask() -> Escrow {
    Escrow {
        chain_id: 11_155_111,
        url: b"https://agent.example/escrow".to_vec(),
        listing_key: [1; 32],
        release_key: [2; 32],
        buyer_pk: [3; 32],
        solution: vec![9; 16],
    }
}

#[test]
fn an_escrow_request_round_trips_exactly() {
    let e = ask();
    let b = e.encode().unwrap();
    assert_eq!(b.len(), 1 + 8 + 2 + 28 + 96 + 2 + 16);
    assert_eq!((b[0], &b[1..9]), (FETCH_ESCROW, &11_155_111u64.to_le_bytes()[..]));
    assert_eq!(Fetch::parse(&b), Some(Fetch::Escrow(e.clone())));
    assert_eq!(Fetch::Escrow(e.clone()).encode(), Some(b.clone()));
    for n in 0..b.len() {
        assert_eq!(Fetch::parse(&b[..n]), None, "cut at {n}");
    }
    assert_eq!(Fetch::parse(&[b.as_slice(), &[0]].concat()), None, "a trailing byte");
}

#[test]
fn the_agent_gets_the_request_without_its_url() {
    let e = ask();
    let body = e.agent_body().unwrap();
    let b = e.encode().unwrap();
    assert_eq!(body[..8], b[1..9]);
    assert_eq!(body[8..], b[1 + 8 + 2 + 28..]);
    assert_eq!((body[8], body[40], body[72]), (1, 2, 3));
}

#[test]
fn lengths_past_their_limits_are_refused() {
    let long_url = Escrow { url: vec![b'a'; MAX_URL + 1], ..ask() };
    assert_eq!(long_url.encode(), None);
    let long_sol = Escrow { solution: vec![0; MAX_SOLUTION + 1], ..ask() };
    assert_eq!(long_sol.encode(), None);
    let mut b = Escrow { solution: vec![0; MAX_SOLUTION], ..ask() }.encode().unwrap();
    assert!(Fetch::parse(&b).is_some(), "MAX_SOLUTION itself is fine");
    let at = b.len() - MAX_SOLUTION - 2;
    b[at..at + 2].copy_from_slice(&((MAX_SOLUTION + 1) as u16).to_le_bytes());
    b.push(0);
    assert_eq!(Fetch::parse(&b), None, "a solution length past MAX_SOLUTION");
}
