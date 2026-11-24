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

//! gtk_prep's one-pass scan for GTK's libraries answers as the three scans
//! it replaced did, at the start, the end and across every near miss.

#[path = "../../../capsule_linux/src/linux/gtk_prep/links.rs"]
mod links;

use links::{names_gtk, LINKS};

/// The rule it replaced, verbatim.
fn three_scans(program: &[u8]) -> bool {
    LINKS.iter().any(|l| program.windows(l.len()).any(|w| w == *l))
}

#[test]
fn each_name_is_found_wherever_it_stands() {
    for name in LINKS {
        for (before, after) in [(&b""[..], &b""[..]), (b"\0\x7fELF", b"\0"), (b"libglibg", b"x")] {
            let program = [before, name, after].concat();
            assert!(names_gtk(&program), "{:?}", String::from_utf8_lossy(&program));
        }
    }
}

#[test]
fn near_misses_and_cut_names_are_not_gtk() {
    for program in [
        &b""[..],
        b"lib",
        b"libg",
        b"libgtk-3.s",
        b"libgtk-2.0.so",
        b"libglib-2.0.so.0",
        b"libgdk_pixbuf-2.0.s",
        b"LIBGTK-3.SO",
    ] {
        assert!(!names_gtk(program), "{:?}", String::from_utf8_lossy(program));
    }
}

#[test]
fn it_answers_as_the_three_scans_did() {
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    let alphabet = b"libgtk-34.sodpxbuf_20\0";
    for _ in 0..2000 {
        let len = (state % 64) as usize;
        let program: Vec<u8> = (0..len)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                alphabet[(state % alphabet.len() as u64) as usize]
            })
            .collect();
        assert_eq!(names_gtk(&program), three_scans(&program), "{program:?}");
    }
    for name in LINKS {
        let program = [&b"padding libg"[..], name].concat();
        assert_eq!(names_gtk(&program), three_scans(&program));
    }
}
