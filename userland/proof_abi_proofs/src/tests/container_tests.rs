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

use nonos_proof_abi::{encode, parse, tag, Kind, MAX_BYTES};

pub fn sections() -> Vec<(u16, Vec<u8>)> {
    vec![
        (tag::RECORD, vec![0; 240]),
        (tag::DEVICE, b"d".to_vec()),
        (tag::ISOLATION, b"i".to_vec()),
        (tag::PUBLICS, vec![0; tag::PUBLICS_BYTES]),
        (tag::MEASUREMENTS, vec![0; 32]),
        (tag::POLICY, vec![0; 256]),
    ]
}

pub fn sample() -> Vec<u8> {
    let s = sections();
    let refs: Vec<(u16, &[u8])> = s.iter().map(|(t, b)| (*t, &b[..])).collect();
    encode(Kind::Isolation, 20_372, &refs).expect("a container")
}

#[test]
fn what_encode_writes_parse_reads() {
    let b = sample();
    let c = parse(&b).expect("parses");
    assert_eq!((c.kind, c.window), (Kind::Isolation, 20_372));
    assert_eq!(c.get(tag::RECORD).map(<[u8]>::len), Some(240));
    assert_eq!((c.get(tag::DEVICE), c.get(tag::ATTEST)), (Some(&b"d"[..]), None));
    let only = encode(Kind::Authenticity, 1, &[(tag::ATTEST, b"t")]).expect("kind 3");
    assert_eq!(parse(&only).map(|c| c.kind), Some(Kind::Authenticity));
}

#[test]
fn encode_takes_exactly_the_kinds_sections() {
    let s = sections();
    let refs: Vec<(u16, &[u8])> = s.iter().map(|(t, b)| (*t, &b[..])).collect();
    let mut short = refs.clone();
    short.remove(4);
    let mut extra = refs.clone();
    extra.insert(4, (tag::ATTEST, b"t"));
    let mut swapped = refs.clone();
    swapped.swap(1, 2);
    for bad in [short, extra, swapped] {
        assert_eq!(encode(Kind::Isolation, 1, &bad), None);
    }
    assert_eq!(encode(Kind::Delivery, 1, &refs), None, "another kind's sections");
    let big = vec![1u8; MAX_BYTES];
    let mut huge = refs.clone();
    huge[1] = (tag::DEVICE, &big);
    assert_eq!(encode(Kind::Isolation, 1, &huge), None);
}

#[test]
fn every_header_field_is_checked() {
    let b = sample();
    for (at, v) in [(0, b'X'), (4, 2), (6, 3), (6, 0), (8, 1), (18, 5), (18, 7)] {
        let mut g = b.clone();
        g[at] = v;
        assert!(parse(&g).is_none(), "byte {at} = {v}");
    }
}
