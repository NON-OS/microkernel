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

//! Version 2: each file's bytes start on a page, one digest a page sits in
//! the head (so the trailer over the head binds it), and a page is served
//! only as its digest. Version 1 packs still read.

mod common;

use common::{src, vimish};
use nonos_linux_pack::{
    build, build_version, Index, Kind, Pack, Refused, Source, HEADER_LEN, PAGE,
};

/// 3.5 pages, each page's bytes different from every other's.
fn big() -> Vec<u8> {
    (0..PAGE * 7 / 2).map(|i| (i / PAGE * 31 + i % 251) as u8).collect()
}

fn with_big(b: &[u8]) -> Vec<Source<'_>> {
    let mut s = vimish();
    s.push(src("usr/lib/libbig.so", Kind::File, false, b));
    s.push(src("usr/lib/libtwin.so", Kind::File, false, b));
    s
}

fn served(p: &Pack<'_>, name: &[u8]) -> Vec<Result<Vec<u8>, Refused>> {
    let e = p.find(name).unwrap();
    (0..e.pages.len()).map(|n| p.page(&e, n).map(|b| b.to_vec())).collect()
}

#[test]
fn a_file_of_a_page_or_more_starts_on_a_page_and_its_pages_rebuild_it() {
    let b = big();
    let bytes = build(with_big(&b)).unwrap();
    let p = Pack::parse(&bytes).unwrap();
    assert!(p.paged());
    for i in 0..p.len() {
        let e = p.entry(i).unwrap();
        if e.kind == Kind::File && !e.data.is_empty() {
            let align = if e.data.len() >= PAGE { PAGE } else { 64 };
            assert_eq!(e.data.start % align, 0, "{:?}", e.name);
            assert_eq!(e.pages.len(), e.data.len().div_ceil(PAGE));
            let whole: Vec<u8> = served(&p, e.name).into_iter().flat_map(|r| r.unwrap()).collect();
            assert_eq!(whole, p.file(&e).unwrap());
        }
    }
    assert_eq!(p.find(b"usr/lib/libbig.so").unwrap().pages.len(), 4);
    assert!(p.find(b"usr/share/vim").unwrap().pages.is_empty());
    assert!(p.find(b"usr/bin/vi").unwrap().pages.is_empty());
}

#[test]
fn the_page_digests_are_in_the_head() {
    let b = big();
    let bytes = build(with_big(&b)).unwrap();
    let head = Index::data_start(&bytes[..HEADER_LEN], bytes.len()).unwrap();
    let table = u64::from_le_bytes(bytes[56..64].try_into().unwrap()) as usize;
    let p = Index::parse(&bytes[..head], bytes.len()).unwrap();
    let pages: usize = (0..p.len()).map(|i| p.entry(i).unwrap().pages.len()).sum();
    assert_eq!(table, pages * 16);
    // A changed digest is a changed head, which the trailer over it refuses;
    // read without that check, it refuses its page.
    let mut bad = bytes.clone();
    let e = Pack::parse(&bytes).unwrap().find(b"usr/lib/libbig.so").unwrap();
    let names_end = {
        let p = Pack::parse(&bytes).unwrap();
        (0..p.len())
            .map(|i| p.entry(i).unwrap())
            .map(|e| e.name.as_ptr() as usize - bytes.as_ptr() as usize + e.name.len())
            .max()
            .unwrap()
    };
    let slot = names_end + (e.pages.start + 2) * 16;
    assert!(slot + 16 <= head);
    bad[slot] ^= 1;
    let p = Pack::parse(&bad).unwrap();
    let pages = served(&p, b"usr/lib/libbig.so");
    assert_eq!(pages.iter().map(Result::is_ok).collect::<Vec<_>>(), [true, true, false, true]);
}

#[test]
fn a_byte_changed_in_one_page_refuses_that_page_alone() {
    let b = big();
    let bytes = build(with_big(&b)).unwrap();
    let e = Pack::parse(&bytes).unwrap().find(b"usr/lib/libbig.so").unwrap();
    for page in 0..e.pages.len() {
        for at in [0, 1, PAGE / 2, PAGE - 1] {
            let off = page * PAGE + at;
            if off >= e.data.len() {
                continue;
            }
            let mut bad = bytes.clone();
            bad[e.data.start + off] ^= 0x40;
            let p = Pack::parse(&bad).unwrap();
            let got = served(&p, b"usr/lib/libbig.so");
            for (n, r) in got.iter().enumerate() {
                assert_eq!(r.is_err(), n == page, "byte {off}, page {n}");
            }
            assert_eq!(p.file(&e), Err(Refused::Digest));
            assert!(served(&p, b"usr/lib/libtwin.so").iter().all(Result::is_ok));
        }
    }
}

#[test]
fn reordered_or_borrowed_pages_are_refused() {
    let b = big();
    let bytes = build(with_big(&b)).unwrap();
    let p = Pack::parse(&bytes).unwrap();
    let (e, twin) = (p.find(b"usr/lib/libbig.so").unwrap(), p.find(b"usr/lib/libtwin.so").unwrap());
    let page = |e: &nonos_linux_pack::Entry, n: usize| p.page_bytes(e, n).unwrap();
    // Pages 0 and 1 of one file swapped.
    let mut swapped = bytes.clone();
    let (a, c) = (page(&e, 0), page(&e, 1));
    swapped[a.clone()].copy_from_slice(&bytes[c.clone()]);
    swapped[c].copy_from_slice(&bytes[a]);
    let s = Pack::parse(&swapped).unwrap();
    assert_eq!(
        served(&s, b"usr/lib/libbig.so").iter().map(Result::is_ok).collect::<Vec<_>>(),
        [false, false, true, true]
    );
    // A page of another file with other bytes, put in its place.
    let other = p.find(b"usr/share/vimrc").unwrap();
    let mut borrowed = bytes.clone();
    let from = page(&other, 0);
    let to = page(&e, 1);
    let n = from.len().min(to.len());
    borrowed[to.start..to.start + n].copy_from_slice(&bytes[from.start..from.start + n]);
    let s = Pack::parse(&borrowed).unwrap();
    assert!(served(&s, b"usr/lib/libbig.so")[1].is_err());
    // The same bytes at the same place in a twin are that twin's: a page is
    // bound to its file and number by its slot in the table.
    assert_eq!(served(&p, b"usr/lib/libtwin.so"), served(&p, b"usr/lib/libbig.so"));
    assert_ne!(e.pages, twin.pages);
    assert!(p.verify_page(&e, 0, &bytes[page(&e, 1)]).is_err());
    assert!(p.verify_page(&e, 3, &bytes[page(&e, 3)][..1]).is_err(), "a cut page");
}

#[test]
fn a_byte_in_the_padding_after_a_page_refuses_the_pack_not_a_page() {
    let b = big();
    let bytes = build(with_big(&b)).unwrap();
    let e = Pack::parse(&bytes).unwrap().find(b"usr/lib/libbig.so").unwrap();
    let mut bad = bytes.clone();
    bad[e.data.end] = 1;
    assert_eq!(Pack::parse(&bad).err(), Some(Refused::Data));
}

#[test]
fn a_page_number_out_of_order_is_refused() {
    let b = big();
    let bytes = build(with_big(&b)).unwrap();
    let p = Pack::parse(&bytes).unwrap();
    let i = (0..p.len()).find(|&i| p.entry(i).unwrap().name == b"usr/lib/libtwin.so").unwrap();
    for v in [0u64, 1, 1 << 40] {
        let mut bad = bytes.clone();
        let at = HEADER_LEN + i * 48 + 40;
        bad[at..at + 8].copy_from_slice(&v.to_le_bytes());
        assert!(Pack::parse(&bad).is_err(), "first page {v}");
    }
}

#[test]
fn pages_over_a_range_cut_at_the_files_end() {
    let b = big();
    let bytes = build(with_big(&b)).unwrap();
    let p = Pack::parse(&bytes).unwrap();
    let e = p.find(b"usr/lib/libbig.so").unwrap();
    assert_eq!(p.pages_over(&e, 0, 832), 0..1);
    assert_eq!(p.pages_over(&e, PAGE - 1, 2), 0..2);
    assert_eq!(p.pages_over(&e, PAGE, PAGE), 1..2);
    assert_eq!(p.pages_over(&e, 0, usize::MAX), 0..4);
    assert_eq!(p.pages_over(&e, e.data.len(), 1), 0..0);
    assert_eq!(p.pages_over(&e, 5, 0), 0..0);
}

#[test]
fn version_1_packs_still_read_and_carry_no_page_digests() {
    let b = big();
    let v1 = build_version(with_big(&b), 1).unwrap();
    let v2 = build(with_big(&b)).unwrap();
    assert_eq!(&v1[..8], b"NONOSPK1");
    assert_eq!(&v2[..8], b"NONOSPK2");
    let (p1, p2) = (Pack::parse(&v1).unwrap(), Pack::parse(&v2).unwrap());
    assert!(!p1.paged() && p2.paged());
    assert_eq!(p1.len(), p2.len());
    for i in 0..p1.len() {
        let (a, c) = (p1.entry(i).unwrap(), p2.entry(i).unwrap());
        assert_eq!((a.name, a.kind, a.exec, a.digest), (c.name, c.kind, c.exec, c.digest));
        assert!(a.pages.is_empty());
        if a.kind != Kind::Dir {
            assert_eq!(p1.file(&a).unwrap(), p2.file(&c).unwrap());
        }
    }
    let head = Index::data_start(&v1[..HEADER_LEN], v1.len()).unwrap();
    assert!(Index::parse(&v1[..head], v1.len()).is_ok());
    assert_eq!(build_version(vimish(), 3).err(), Some(Refused::Build));
}

#[test]
fn building_twice_gives_the_same_bytes() {
    let b = big();
    assert_eq!(build(with_big(&b)).unwrap(), build(with_big(&b)).unwrap());
}
