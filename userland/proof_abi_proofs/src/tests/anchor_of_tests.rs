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

//! A kind-1 container's anchor, from its own record and device tag: what
//! fetch uploads under and what a reader holds the relay's bytes to.

use nonos_proof_abi::{anchor, anchor_of, decode_device, encode, record_id, tag, Kind};

use crate::fixture::{honest, LISTING};

#[test]
fn the_anchor_is_the_containers_own() {
    let (f, parts, _) = honest();
    let b = parts.bytes();
    let (st, _) = decode_device(&parts.sections[1].1).expect("device");
    let t = st.tag.map(|w| w.to_u64());
    let want = anchor(Kind::Isolation, &LISTING, &record_id(&f.record.encode()), &t, &b);
    assert_eq!(anchor_of(&LISTING, &b), Some(want));
    assert_ne!(anchor_of(&[0x77; 32], &b), Some(want), "another listing, another anchor");
}

#[test]
fn bytes_that_are_not_a_kind_1_container_have_none() {
    let (_, parts, _) = honest();
    let b = parts.bytes();
    assert_eq!(anchor_of(&LISTING, &b[..b.len() - 1]), None, "torn");
    assert_eq!(anchor_of(&LISTING, &[b.clone(), vec![0]].concat()), None, "trailing");
    assert_eq!(anchor_of(&LISTING, b"GET /etc/passwd HTTP/1.1\r\n\r\n"), None);
    let mut no_device = parts.bytes();
    let at = 20 + 6 + 240 + 6;
    no_device[at..at + 8].copy_from_slice(b"NOTADEVI");
    assert_eq!(anchor_of(&LISTING, &no_device), None, "a device section that is not one");
    let s = |t: u16| parts.sections.iter().find(|s| s.0 == t).expect("section").1.clone();
    let delivery = [(tag::RECORD, s(tag::RECORD)), (tag::DEVICE, s(tag::DEVICE))];
    let rest = [(tag::DELIVERY, vec![0u8; 208]), (tag::MEASUREMENTS, s(tag::MEASUREMENTS))];
    let refs: Vec<(u16, &[u8])> = delivery.iter().chain(&rest).map(|(t, v)| (*t, &v[..])).collect();
    let kind2 = encode(Kind::Delivery, 20_000, &refs).expect("a kind-2 container");
    assert_eq!(anchor_of(&LISTING, &kind2), None, "kind 2 is never anchored");
}
