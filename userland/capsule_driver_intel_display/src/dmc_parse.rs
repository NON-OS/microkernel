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

//! Turns the sealed DMC list into a table at compile time: one blob name
//! and its SHA-256 in 64 lowercase hex digits per line. A line that does
//! not parse stops the build.

pub struct Entry {
    pub name: [u8; 16],
    pub name_len: usize,
    pub digest: [u8; 32],
}

pub struct Sealed {
    pub entries: [Entry; 16],
    pub len: usize,
}

const fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        _ => panic!("lowercase hex digest expected"),
    }
}

pub const fn parse(src: &[u8]) -> Sealed {
    const EMPTY: Entry = Entry { name: [0; 16], name_len: 0, digest: [0; 32] };
    let mut t = Sealed { entries: [EMPTY; 16], len: 0 };
    let mut at = 0;
    while at < src.len() {
        if src[at] == b'#' || src[at] == b'\n' {
            while at < src.len() && src[at] != b'\n' {
                at += 1;
            }
            at += 1;
            continue;
        }
        assert!(t.len < 16, "too many sealed blobs");
        let e = &mut t.entries[t.len];
        while src[at] != b' ' {
            assert!(e.name_len < 16, "blob name too long");
            e.name[e.name_len] = src[at];
            e.name_len += 1;
            at += 1;
        }
        at += 1;
        assert!(at + 64 < src.len() && src[at + 64] == b'\n', "digest line");
        let mut i = 0;
        while i < 32 {
            e.digest[i] = nibble(src[at + 2 * i]) << 4 | nibble(src[at + 2 * i + 1]);
            i += 1;
        }
        t.len += 1;
        at += 65;
    }
    t
}
