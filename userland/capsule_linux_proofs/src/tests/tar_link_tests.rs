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


//! Every typeflag a package carries survives the walk, and what is dropped is
//! counted. `links.tar` is three archives end to end, as an apk is, written by
//! Python's tarfile in ustar, pax and GNU form.

use crate::install::tar::{walk, Kind};

const LIBFFI: &[u8] = include_bytes!("../../vectors/libffi.tar");
const LINKS: &[u8] = include_bytes!("../../vectors/links.tar");

fn link_of(name: &[u8], data: &[u8]) -> Option<Vec<u8>> {
    walk(data).entries.into_iter().find(|e| e.name == name).and_then(|e| match e.kind {
        Kind::Symlink(t) | Kind::Hardlink(t) => Some(t),
        _ => None,
    })
}

#[test]
fn the_soname_link_in_a_real_package_is_kept() {
    let to = link_of(b"usr/lib/libffi.so.8", LIBFFI).expect("the soname link");
    assert_eq!(to, b"libffi.so.8.1.4");
    assert_eq!(walk(LIBFFI).dropped, 0, "a real package loses nothing");
}

#[test]
fn symbolic_and_hard_links_keep_their_targets() {
    assert_eq!(link_of(b"usr/lib/libz.so.1", LINKS).as_deref(), Some(&b"libz.so.1.3"[..]));
    let hard = walk(LINKS).entries.into_iter().find(|e| e.name == b"usr/lib/libz-copy.so");
    assert!(matches!(hard.map(|e| e.kind), Some(Kind::Hardlink(t)) if t == b"usr/lib/libz.so.1.3"));
}

#[test]
fn long_paths_arrive_whole_in_every_form() {
    let names: Vec<Vec<u8>> = walk(LINKS).entries.into_iter().map(|e| e.name).collect();
    for want in [
        format!("usr/share/{}/deep.txt", "p".repeat(120)),
        format!("usr/share/{}/deep.txt", "g".repeat(120)),
        format!("usr/lib/{}/libq.so.1.0", "q".repeat(95)),
    ] {
        assert!(names.contains(&want.clone().into_bytes()), "missing {}", &want[..30]);
    }
    let pax = link_of(b"usr/bin/short", LINKS).expect("a pax linkpath");
    assert_eq!(pax, format!("/usr/share/{}", "t".repeat(120)).into_bytes());
}

#[test]
fn devices_and_fifos_are_counted_not_silently_lost() {
    let w = walk(LINKS);
    assert_eq!(w.dropped, 2, "one fifo and one character device");
    assert!(w.entries.iter().all(|e| !e.name.starts_with(b"dev/")));
}

#[test]
fn file_bodies_are_the_bytes_written() {
    let w = walk(LINKS);
    let body = |n: &[u8]| w.entries.iter().find(|e| e.name == n).map(|e| e.body.clone());
    assert_eq!(body(b"usr/lib/libz.so.1.3").as_deref(), Some(&b"\x7fELF"[..]));
    assert!(matches!(w.entries.iter().find(|e| e.name == b"usr/lib").map(|e| &e.kind), Some(Kind::Dir)));
}
