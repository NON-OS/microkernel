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

//! A symbolic link never stands in for sealed bytes, even when it points at
//! the very same bytes: the check reads inside the release or not at all.

use super::fixture::Release;
use super::sealed_tests::fails_naming;
use std::fs;
use std::os::unix::fs::symlink;

#[test]
fn a_link_in_place_of_a_sealed_file_fails_even_to_the_same_bytes() {
    let (r, elsewhere) = (Release::dev(), tempfile::tempdir().unwrap());
    let img = elsewhere.path().join("nonos.img");
    fs::rename(r.path().join("nonos.img"), &img).unwrap();
    symlink(&img, r.path().join("nonos.img")).unwrap();
    fails_naming(&r, "symbolic link");
}

#[test]
fn a_link_inside_the_esp_fails() {
    let r = Release::dev();
    fs::remove_file(r.path().join("esp/kernel.bin")).unwrap();
    symlink(r.path().join("kernel.bin"), r.path().join("esp/kernel.bin")).unwrap();
    fails_naming(&r, "link or a special file");
}

#[test]
fn a_linked_directory_in_place_of_the_esp_fails() {
    let (r, elsewhere) = (Release::dev(), tempfile::tempdir().unwrap());
    let esp = elsewhere.path().join("esp");
    fs::rename(r.path().join("esp"), &esp).unwrap();
    symlink(&esp, r.path().join("esp")).unwrap();
    fails_naming(&r, "symbolic link");
}
