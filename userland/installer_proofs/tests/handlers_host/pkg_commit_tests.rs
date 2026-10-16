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

//! pkg_commit: what it installs from a package at MAX_PACKAGE, and the packages it refuses.

use crate::alloc_watch::held_after;
use crate::clients::vfs;
use crate::frame::{einval, reply};
use crate::packages::{elf, elf_len_of, package, package_of, past_max, MAX_PACKAGE};
use crate::packages::{CERT, MANIFEST, ZK};
use crate::pkg_frame::{commit, digest, fresh, stored, EACCES, PATH, STORED};
use crate::process::INSTALLER_PID;
use crate::{services, verify};

#[test]
fn a_verified_package_of_max_package_bytes_is_installed_whole() {
    fresh();
    let pkg = package_of(MAX_PACKAGE);
    vfs::put(PATH, pkg.clone());
    assert_eq!(commit(&digest(&pkg), PATH), reply(0, &[]));
    let (paths, blobs): (Vec<_>, Vec<_>) = vfs::installs().into_iter().unzip();
    assert_eq!(paths, STORED.map(stored));
    assert!(blobs[0] == elf(elf_len_of(MAX_PACKAGE)), "the elf was not installed whole");
    assert_eq!(blobs[1..], [MANIFEST, CERT, ZK]);
    assert_eq!(services::asked(), [&b"app.demo_pkg"[..], b"demo_pkg"]);
}

// The digest committed is that of past_max cut to MAX_PACKAGE, the package a
// cut read would verify, so nothing but the length check refuses it.
#[test]
fn a_package_past_max_package_is_refused_with_nothing_installed() {
    fresh();
    let pkg = past_max();
    vfs::put(PATH, pkg.clone());
    let cut = digest(&pkg[..MAX_PACKAGE]);
    let (answer, held) = held_after(|| commit(&cut, PATH));
    assert_eq!(answer, einval());
    assert!(verify::calls().is_empty(), "a package past MAX_PACKAGE reached verify");
    assert!(vfs::installs().is_empty(), "a package past MAX_PACKAGE was installed");
    assert!(held < 1 << 20, "{held} bytes still held after the reply");
    let cap = MAX_PACKAGE as u32 + 1;
    assert_eq!(vfs::reads(), [(INSTALLER_PID, PATH.to_vec(), cap)]);
}

#[test]
fn a_verified_package_whose_digest_is_not_the_committed_one_is_refused() {
    fresh();
    let pkg = package(64);
    vfs::put(PATH, pkg.clone());
    let mut consented = digest(&pkg);
    consented[31] ^= 1;
    assert_eq!(commit(&consented, PATH), reply(EACCES, &[]));
    assert_eq!(verify::calls().len(), 1, "the package was not verified");
    assert!(vfs::installs().is_empty(), "a package past its consent was installed");
}
