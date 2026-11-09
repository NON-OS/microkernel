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

//! A packed file read a page at a time, as a descriptor reads it: only the
//! pages a read covers are fetched, each from a real version 2 pack and
//! checked against its page digest, kept once fetched, and a changed page
//! refuses the reads that cover it and no other.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::Range;

use nonos_linux_pack::{build, Kind, Pack, Source, PAGE};

use super::pages::{assemble, pages_over};

const NAME: &[u8] = b"usr/lib/libbig.so.1";

fn body() -> Vec<u8> {
    (0..PAGE * 5 + 1234).map(|i| (i * 7 + i / PAGE) as u8).collect()
}

fn pack(b: &[u8]) -> Vec<u8> {
    let s = vec![
        Source { name: b"usr/lib/libbig.so.1".to_vec(), kind: Kind::File, exec: false, bytes: b },
        Source { name: b"usr/lib/small".to_vec(), kind: Kind::File, exec: false, bytes: b"tiny" },
    ];
    build(s).unwrap()
}

/// Reads through a fresh descriptor, counting fetches and pages fetched.
struct Reader<'a> {
    pack: Pack<'a>,
    got: BTreeMap<usize, Vec<u8>>,
    fetches: RefCell<Vec<Range<usize>>>,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Reader {
            pack: Pack::parse(bytes).unwrap(),
            got: BTreeMap::new(),
            fetches: RefCell::new(Vec::new()),
        }
    }

    fn read(&mut self, offset: u64, len: u32) -> Result<Vec<u8>, &'static str> {
        let e = self.pack.find(NAME).unwrap();
        let (pack, fetches) = (&self.pack, &self.fetches);
        let fetch = |pages: Range<usize>| {
            fetches.borrow_mut().push(pages.clone());
            pages
                .map(|n| pack.page(&e, n).map(|b| b.to_vec()).map_err(|_| "not its digest"))
                .collect()
        };
        assemble(e.data.len() as u64, offset, len, &mut self.got, fetch, "cut")
    }
}

#[test]
fn a_read_returns_the_files_bytes_at_every_offset() {
    let b = body();
    let bytes = pack(&b);
    let mut r = Reader::new(&bytes);
    for (off, len) in
        [(0, 832), (4000, 200), (4096, 4096), (0, u32::MAX), (b.len() as u64 - 5, 50), (100, 0)]
    {
        let end = (off as usize).saturating_add(len as usize).min(b.len());
        assert_eq!(r.read(off, len).unwrap(), b[off as usize..end], "{off} {len}");
    }
    assert_eq!(r.read(b.len() as u64, 10).unwrap(), b"");
}

#[test]
#[allow(clippy::single_range_in_vec_init)]
fn only_the_pages_a_read_covers_are_fetched_and_each_once() {
    let b = body();
    let bytes = pack(&b);
    let mut r = Reader::new(&bytes);
    r.read(0, 832).unwrap();
    assert_eq!(*r.fetches.borrow(), [0..1], "an ELF header is one page");
    r.read(3 * PAGE as u64 + 10, 10).unwrap();
    r.read(0, u32::MAX).unwrap();
    assert_eq!(*r.fetches.borrow(), [0..1, 3..4, 1..3, 4..6], "runs of pages not held");
    r.read(0, u32::MAX).unwrap();
    assert_eq!(r.fetches.borrow().len(), 4, "held pages are not fetched again");
}

#[test]
fn a_changed_page_refuses_the_reads_over_it_alone() {
    let b = body();
    let mut bytes = pack(&b);
    let start = Pack::parse(&bytes).unwrap().find(NAME).unwrap().data.start;
    bytes[start + 2 * PAGE + 7] ^= 1;
    let mut r = Reader::new(&bytes);
    assert_eq!(r.read(0, 2 * PAGE as u32).unwrap(), b[..2 * PAGE]);
    assert!(r.read(2 * PAGE as u64, 1).is_err());
    assert!(r.read(0, u32::MAX).is_err());
    assert_eq!(r.read(3 * PAGE as u64, 100).unwrap(), b[3 * PAGE..3 * PAGE + 100]);
    assert!(!r.got.contains_key(&2), "a refused page is never kept");
}

#[test]
fn a_page_of_the_wrong_length_is_refused() {
    let mut got = BTreeMap::new();
    let short = |p: Range<usize>| Ok::<_, &str>(p.map(|_| vec![0u8; 100]).collect());
    assert_eq!(assemble(3 * PAGE as u64, 0, 10, &mut got, short, "cut"), Err("cut"));
    let fewer = |_: Range<usize>| Ok::<_, &str>(Vec::new());
    assert_eq!(assemble(3 * PAGE as u64, 0, 10, &mut got, fewer, "cut"), Err("cut"));
    assert!(got.is_empty());
}

#[test]
fn pages_over_matches_the_packs_own_count() {
    let b = body();
    let bytes = pack(&b);
    let p = Pack::parse(&bytes).unwrap();
    let e = p.find(NAME).unwrap();
    for (off, len) in [(0u64, 1u32), (4095, 2), (0, u32::MAX), (8192, 4096), (99999, 5)] {
        assert_eq!(
            pages_over(e.data.len() as u64, off, len),
            p.pages_over(&e, off as usize, len as usize)
        );
    }
}
