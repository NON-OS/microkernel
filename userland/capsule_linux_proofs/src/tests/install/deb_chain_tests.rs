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

//! The chain an install walks, on an archive Debian's own tools made:
//! Release.gpg over Release, Release over Packages, Packages over each .deb.

use nonos_openpgp::{dearmor, keys, verify};

use super::service::Service;
use crate::install::deb::packages::stanzas;
use crate::install::deb::release::sums;
use crate::install::unpacked::decompressed;

pub const ARCHIVE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/vectors/deb");

pub fn read(path: &str) -> Vec<u8> {
    std::fs::read(format!("{ARCHIVE}/{path}")).expect(path)
}

#[test]
fn the_release_verifies_under_the_archive_key_only() {
    let ring = keys(&dearmor(&read("archive-key.asc")).expect("armor")).expect("key");
    let sig = dearmor(&read("dists/nonos/Release.gpg")).expect("armor");
    let release = read("dists/nonos/Release");
    assert!(verify(&Service, &ring, &sig, &release).is_ok());
    let mut changed = release.clone();
    changed[0] ^= 0x20;
    assert!(verify(&Service, &ring, &sig, &changed).is_err(), "a changed Release");
}

#[test]
fn every_index_the_release_lists_matches_it() {
    let listed = sums(&String::from_utf8(read("dists/nonos/Release")).expect("text"));
    assert_eq!(listed.len(), 2);
    for s in listed {
        let bytes = read(&format!("dists/nonos/{}", s.path));
        assert_eq!((bytes.len(), nonos_hash::sha256(&bytes)), (s.size, s.sha256), "{}", s.path);
    }
}

#[test]
fn the_index_names_every_package_by_its_real_checksum() {
    let packed = decompressed(&read("dists/nonos/main/binary-amd64/Packages.xz")).expect("xz");
    assert_eq!(packed, read("dists/nonos/main/binary-amd64/Packages"));
    let records = stanzas(&String::from_utf8(packed).expect("text"));
    assert_eq!(records.len(), 3);
    for r in &records {
        assert_eq!(Some(nonos_hash::sha256(&read(&r.filename))), r.sha256, "{}", r.name);
    }
    let hello = records.iter().find(|r| r.name == "nonos-hello").expect("nonos-hello");
    assert_eq!(
        hello.depends,
        [vec!["libnonos1", "libnonos-alt"], vec!["missing-alt", "libnonos-virtual"]]
    );
    let lib = records.iter().find(|r| r.name == "libnonos1").expect("libnonos1");
    assert_eq!(lib.provides, ["libnonos-virtual"]);
}
