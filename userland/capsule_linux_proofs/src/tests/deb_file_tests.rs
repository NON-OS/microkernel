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

//! Each .deb opens to exactly the files dpkg-deb was given, whichever of
//! xz, zstd or gzip its data member is, with dpkg's `./` gone from names.

use super::deb_chain_tests::read;
use crate::install::deb::ar::members;
use crate::install::tar::walk;
use crate::install::tar_kind::Kind;
use crate::install::unpacked::unpacked;

fn opened(path: &str) -> crate::install::tar::Walk {
    let deb = read(path);
    let parts = members(&deb).expect("an ar archive");
    let names: Vec<&[u8]> = parts.iter().map(|(n, _)| *n).collect();
    assert_eq!(parts[0], (b"debian-binary".as_slice(), b"2.0\n".as_slice()));
    assert!(names[1].starts_with(b"control.tar") && names[2].starts_with(b"data.tar"), "{path}");
    walk(&unpacked(parts[2].1).expect("the data member decompresses"))
}

fn file<'a>(w: &'a crate::install::tar::Walk, name: &str) -> Option<&'a [u8]> {
    w.entries
        .iter()
        .find(|e| e.name == name.as_bytes() && matches!(e.kind, Kind::File))
        .map(|e| e.body.as_slice())
}

#[test]
fn an_xz_data_member_opens() {
    let w = opened("pool/main/n/nonos-hello/nonos-hello_1.0_amd64.deb");
    assert_eq!(file(&w, "usr/bin/nonos-hello"), Some(b"\x7fELF nonos hello\n".as_slice()));
    assert_eq!(w.dropped, 0);
}

#[test]
fn a_zstd_data_member_opens_with_its_link() {
    let w = opened("pool/main/l/libnonos1/libnonos1_1.0_amd64.deb");
    assert_eq!(file(&w, "usr/lib/libnonos.so.1"), Some(b"\x7fELF libnonos\n".as_slice()));
    let link = w.entries.iter().find(|e| e.name == b"usr/lib/libnonos.so").expect("the link");
    assert!(matches!(&link.kind, Kind::Symlink(to) if to == b"libnonos.so.1"));
}

#[test]
fn a_gzip_data_member_opens() {
    let w = opened("pool/main/n/nonos-gz/nonos-gz_1.0_amd64.deb");
    assert_eq!(file(&w, "usr/share/nonos/gz"), Some(b"gzip member\n".as_slice()));
    assert!(w.entries.iter().all(|e| !e.name.starts_with(b"./")), "no ./ survives");
}
