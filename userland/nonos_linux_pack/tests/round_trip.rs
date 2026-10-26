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

//! What the builder writes, the parser reads back: every entry, by name,
//! with its kind, mode and bytes, and each file only as its digest.

mod common;

use common::vimish;
use nonos_linux_pack::{build, Kind, Pack, Refused};

fn pack() -> Vec<u8> {
    build(vimish()).expect("built")
}

#[test]
fn every_source_reads_back_as_it_went_in() {
    let bytes = pack();
    let p = Pack::parse(&bytes).expect("parsed");
    let sources = vimish();
    assert_eq!(p.len(), sources.len());
    for s in &sources {
        let e = p.find(&s.name).expect("found");
        assert_eq!((e.kind, e.exec), (s.kind, s.exec), "{:?}", s.name);
        if s.kind != Kind::Dir {
            assert_eq!(p.file(&e).unwrap(), &s.bytes[..]);
        }
    }
    assert_eq!(p.find(b"usr/bin/nano"), None);
    assert_eq!(p.find(b"usr/bin"), None, "a directory is only there if named");
}

#[test]
fn under_a_directory_is_its_whole_subtree_and_nothing_beside_it() {
    let bytes = pack();
    let p = Pack::parse(&bytes).unwrap();
    let names = |d: &[u8]| -> Vec<Vec<u8>> {
        p.under(d).map(|i| p.entry(i).unwrap().name.to_vec()).collect()
    };
    let vim = names(b"usr/share/vim");
    assert_eq!(vim.len(), 3, "vim91 and its two files, not vimrc");
    assert!(vim.iter().all(|n| n.starts_with(b"usr/share/vim/")));
    assert_eq!(names(b"usr/bin").len(), 2);
    assert!(names(b"usr/bin/vim").is_empty());
}

#[test]
fn a_byte_changed_in_a_file_is_refused_at_that_file_alone() {
    let mut bytes = pack();
    let at = Pack::parse(&bytes).unwrap().find(b"usr/share/vimrc").unwrap().data.start;
    bytes[at + 10] ^= 1;
    let p = Pack::parse(&bytes).expect("the index still holds");
    let bad = p.find(b"usr/share/vimrc").unwrap();
    assert_eq!(p.file(&bad), Err(Refused::Digest));
    let good = p.find(b"usr/bin/vim").unwrap();
    assert!(p.file(&good).is_ok(), "the other files are still served");
}
