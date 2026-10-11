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

//! The agent reads exactly what settle's encoder sends; nothing else.

use super::super::body::{Asked, BODY_LEN};
use super::super::seen::{Seen, SEEN_MAX};
use super::buyer::request;

#[test]
fn the_agent_reads_settles_body_exactly() {
    let (_, req) = request(1);
    let b = req.agent_body();
    let a = Asked::parse(&b).unwrap();
    assert_eq!(b.len(), BODY_LEN);
    assert_eq!((a.chain_id, a.buyer_pk, &a.solution[..]), (1, req.buyer_pk, &req.solution[..]));
    assert_eq!(Asked::parse(&[b.as_slice(), &[0]].concat()), None);
    let mut long = b.clone();
    long[104] = 17;
    assert_eq!(Asked::parse(&long), None, "a solution length other than 16");
}

#[test]
fn seen_forgets_only_past_its_cap() {
    let mut s = Seen::default();
    let pk = [0u8; 32];
    let sol = |i: u32| {
        let mut x = [0u8; 16];
        x[..4].copy_from_slice(&i.to_le_bytes());
        x
    };
    assert!(s.take(&sol(0), &pk) && !s.take(&sol(0), &pk));
    assert!(s.take(&sol(0), &[1; 32]), "another buyer's key is another request");
    for i in 1..=SEEN_MAX as u32 {
        s.take(&sol(i), &pk);
    }
    assert!(s.take(&sol(0), &pk), "the oldest was forgotten");
    assert!(!s.take(&sol(SEEN_MAX as u32), &pk));
}
