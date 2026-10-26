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

//! A pack read as a run reads it: its first 64 bytes, then its index, and
//! a file's bytes from the store only when one is served, checked then.

mod common;

use common::vimish;
use nonos_linux_pack::{build, Index, Refused, HEADER_LEN};

#[test]
fn the_index_alone_finds_a_file_and_its_bytes_are_checked_when_read() {
    let pack = build(vimish()).unwrap();
    let head = Index::data_start(&pack[..HEADER_LEN], pack.len()).unwrap();
    let index = Index::parse(&pack[..head], pack.len()).expect("the index alone");
    let e = index.find(b"usr/share/vim/vim91/defaults.vim").unwrap();
    assert_eq!(index.verify(&e, &pack[e.data.clone()]), Ok(&b"set nocompatible\n"[..]));
    let mut read = pack[e.data.clone()].to_vec();
    read[0] ^= 1;
    assert_eq!(index.verify(&e, &read), Err(Refused::Digest), "a changed byte");
    assert_eq!(index.verify(&e, &read[1..]), Err(Refused::Digest), "a short read");
}

#[test]
fn an_index_cut_short_or_read_long_is_refused() {
    let pack = build(vimish()).unwrap();
    let head = Index::data_start(&pack[..HEADER_LEN], pack.len()).unwrap();
    assert_eq!(Index::parse(&pack[..head - 1], pack.len()).err(), Some(Refused::Layout));
    assert_eq!(Index::parse(&pack[..head + 1], pack.len()).err(), Some(Refused::Layout));
    assert_eq!(Index::parse(&pack[..head], pack.len() - 1).err(), Some(Refused::Data));
}
