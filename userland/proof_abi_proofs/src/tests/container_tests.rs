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

fn sample() -> Vec<u8> {
    let rec = [0u8; 240];
    encode(Kind::Isolation, &[(tag::RECORD, &rec), (tag::DEVICE, b"d"), (tag::ATTEST, b"t")])
        .expect("a container")
}

#[test]
fn what_encode_writes_parse_reads() {
    let b = sample();
    let c = parse(&b).expect("parses");
    assert_eq!(c.kind, Kind::Isolation);
    assert_eq!(c.get(tag::RECORD).map(<[u8]>::len), Some(240));
    assert_eq!(c.get(tag::DEVICE), Some(&b"d"[..]));
    assert_eq!(c.get(tag::ISOLATION), None);
    assert_eq!(c.get(99), None);
}

#[test]
fn encode_refuses_what_parse_would() {
    let rec = [0u8; 240];
    let bad: [&[(u16, &[u8])]; 5] = [
        &[(tag::DEVICE, b"d"), (tag::RECORD, &rec)],
        &[(tag::DEVICE, b"d"), (tag::DEVICE, b"e")],
        &[(9, b"x")],
        &[(tag::RECORD, &rec[..239])],
        &[(tag::MEASUREMENTS, &rec[..33])],
    ];
    for s in bad {
        assert_eq!(encode(Kind::Isolation, s), None);
    }
    let big = vec![1u8; MAX_BYTES];
    assert_eq!(encode(Kind::Isolation, &[(tag::DEVICE, &big)]), None);
}

#[test]
fn every_header_field_is_checked() {
    let b = sample();
    for (at, v) in [(0, b'X'), (4, 2), (6, 4), (6, 0), (8, 1), (10, 2), (10, 4)] {
        let mut g = b.clone();
        g[at] = v;
        assert!(parse(&g).is_none(), "byte {at} = {v}");
    }
}
