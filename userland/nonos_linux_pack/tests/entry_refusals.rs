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

//! Each entry held to the format, each bound broken on its own: a name
//! that is not a plain path, a kind or mode the format lacks, bytes out of
//! place. The whole pack is refused, with the reason.

mod common;
#[path = "common/edit.rs"]
mod edit;

use edit::{pack, put, refused};
use nonos_linux_pack::{Pack, Refused, ENTRY_LEN, HEADER_LEN};

/// Where the entry named `name` is in the pack.
fn at(name: &[u8]) -> usize {
    let bytes = pack();
    let p = Pack::parse(&bytes).unwrap();
    let i = (0..p.len()).find(|&i| p.entry(i).unwrap().name == name).unwrap();
    HEADER_LEN + i * ENTRY_LEN
}

#[test]
fn a_name_is_a_plain_path_inside_the_names_block() {
    let vimrc = at(b"usr/share/vimrc");
    let name = |b: &[u8]| u32::from_le_bytes(b[vimrc..vimrc + 4].try_into().unwrap()) as usize;
    for bad in [&b"usr/share/../rc"[..], b"/sr/share/vimrc", b"usr//hare/vimrc", b"usr/share/./mrc"]
    {
        let got = refused(|b| {
            let n = name(b);
            put(b, n, bad);
        });
        assert_eq!(got, Refused::Name, "{:?}", bad);
    }
    assert_eq!(refused(|b| put(b, vimrc, &u32::MAX.to_le_bytes())), Refused::Name);
    assert_eq!(refused(|b| put(b, vimrc + 4, &0u16.to_le_bytes())), Refused::Name);
}

#[test]
fn kinds_modes_and_bytes_are_the_formats_own() {
    let (vim, vi, dir) = (at(b"usr/bin/vim"), at(b"usr/bin/vi"), at(b"usr/share/vim"));
    assert_eq!(refused(|b| b[vim + 6] = 3), Refused::Kind);
    assert_eq!(refused(|b| b[vi + 7] = 1), Refused::Kind, "a link is not executable");
    assert_eq!(refused(|b| b[vim + 7] = 2), Refused::Kind);
    assert_eq!(refused(|b| b[dir + 16] = 1), Refused::Data, "a directory with bytes");
    assert_eq!(refused(|b| b[vim + 8] |= 1), Refused::Data, "a file off its alignment");
    assert_eq!(refused(|b| b[vim + 40] = 1), Refused::Layout);
    let vimrc = at(b"usr/share/vimrc");
    let over = |b: &mut Vec<u8>| {
        let start: [u8; 8] = b[vim + 8..vim + 16].try_into().unwrap();
        put(b, vimrc + 8, &start);
    };
    assert_eq!(refused(over), Refused::Data, "two files over the same bytes");
}
