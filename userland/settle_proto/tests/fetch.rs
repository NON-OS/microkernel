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

use nonos_settle_proto::*;

#[test]
fn fetch_forms_round_trip() {
    let url = b"https://pkg.example/x".to_vec();
    let rel = Fetch::Release { url: url.clone(), release_hash: [4; 32] };
    let b = rel.encode().unwrap();
    assert_eq!((b[0], b.len()), (FETCH_RELEASE, 1 + 2 + url.len() + 32));
    assert_eq!(Fetch::parse(&b), Some(rel));
    for n in 0..b.len() {
        assert_eq!(Fetch::parse(&b[..n]), None, "{n}");
    }
    let page = Fetch::Page { release_hash: [4; 32], offset: 61_440 };
    let p = page.encode().unwrap();
    assert_eq!(Fetch::parse(&p), Some(page));
    assert_eq!(Fetch::parse(&[p.as_slice(), &[0]].concat()), None);
    let long = Fetch::Release { url: vec![b'a'; MAX_URL + 1], release_hash: [0; 32] };
    assert_eq!(long.encode(), None);
    assert_eq!(Fetch::parse(&[FETCH_ESCROW, 1, 2]), None, "an escrow body too short");
    assert_eq!(Fetch::parse(&[3]), None);
}

#[test]
fn pages_cover_a_release_and_stay_inside_it() {
    let release: Vec<u8> = (0..150_000u32).map(|i| i as u8).collect();
    let (mut at, mut got) = (0u32, Vec::new());
    while (at as usize) < release.len() {
        let p = Page::parse(&Page::of(&release, at).unwrap().encode()).unwrap();
        assert!(p.bytes.len() <= PAGE_LEN && p.total == 150_000 && p.offset == at);
        got.extend_from_slice(&p.bytes);
        at += p.bytes.len() as u32;
    }
    assert_eq!(got, release);
    assert_eq!(Page::of(&release, 150_000), None, "past the end");
    assert_eq!(Page::of(&[], 0).map(|p| p.total), Some(0));
    assert_eq!(Page::of(&vec![0; MAX_RELEASE + 1], 0), None);
    let mut lying = Page { total: 10, offset: 8, bytes: vec![1; 3] }.encode();
    assert_eq!(Page::parse(&lying), None, "past its total");
    lying[..4].copy_from_slice(&((MAX_RELEASE + 1) as u32).to_le_bytes());
    assert_eq!(Page::parse(&lying), None, "past MAX_RELEASE");
}
