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

//! pkg_query: a package of MAX_PACKAGE bytes is read whole and verified; one byte more is refused unverified.

use crate::alloc_watch::held_after;
use crate::clients::vfs;
use crate::frame::{einval, reply};
use crate::packages::{elf, elf_len_of, package, package_of, past_max, MAX_PACKAGE};
use crate::packages::{CERT, MANIFEST, ZK};
use crate::pkg_frame::{digest, fresh, query, EACCES, PATH};
use crate::process::INSTALLER_PID;
use crate::verify::{self, CAPS, NAME, NS, TIER};

/// digest | caps | tier | name_len | name | ns_len | ns, as pkg_query replies.
fn summary(pkg: &[u8]) -> Vec<u8> {
    let head = [&digest(pkg)[..], &CAPS.to_le_bytes(), &[TIER, NAME.len() as u8]].concat();
    [&head[..], NAME, &[NS.len() as u8], NS].concat()
}

#[test]
fn a_package_of_max_package_bytes_is_read_whole_and_verified() {
    fresh();
    let pkg = package_of(MAX_PACKAGE);
    vfs::put(PATH, pkg.clone());
    assert_eq!(query(PATH), reply(0, &summary(&pkg)));
    let calls = verify::calls();
    assert_eq!(calls.len(), 1);
    assert!(calls[0][0] == elf(elf_len_of(MAX_PACKAGE)), "verify was not given the whole elf");
    assert_eq!(calls[0][1..], [CERT, MANIFEST, ZK]);
}

#[test]
fn a_package_of_max_package_bytes_that_fails_verify_gets_its_status() {
    fresh();
    verify::reset(-1);
    vfs::put(PATH, package_of(MAX_PACKAGE));
    assert_eq!(query(PATH), reply(EACCES, &[]));
    assert_eq!(verify::calls().len(), 1);
}

// read_file stops at the max it is given, so the installer asks for one byte
// more than it takes. past_max cut to MAX_PACKAGE is a package that verifies:
// only the length check stands between it and verify.
#[test]
fn a_package_past_max_package_is_refused_unverified_and_dropped_before_the_reply() {
    fresh();
    vfs::put(PATH, past_max());
    let (answer, held) = held_after(|| query(PATH));
    assert_eq!(answer, einval());
    assert!(verify::calls().is_empty(), "a package past MAX_PACKAGE reached verify");
    assert!(held < 1 << 20, "{held} bytes still held after the reply");
}

#[test]
fn the_package_is_read_as_the_installer_one_byte_past_max_package() {
    fresh();
    vfs::put(PATH, package(64));
    query(PATH);
    let cap = MAX_PACKAGE as u32 + 1;
    assert_eq!(vfs::reads(), [(INSTALLER_PID, PATH.to_vec(), cap)]);
}
