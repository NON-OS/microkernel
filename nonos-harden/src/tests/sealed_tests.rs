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

//! `release.sealed` passes exactly the bytes that were sealed: one flipped
//! byte, or a missing or added file, fails it, and the reason names where.

use super::fixture::Release;
use crate::check::{Check, Status};
use crate::ctx::Ctx;
use crate::release::{load, sealed};
use std::fs;

fn sealed_of(r: &Release) -> Check {
    let mut ctx = Ctx::new(r.path().into(), None);
    assert_eq!(load::check(&mut ctx).status, Status::Pass);
    sealed::check(&mut ctx)
}

pub fn fails_naming(r: &Release, what: &str) {
    let c = sealed_of(r);
    assert_eq!(c.status, Status::Fail, "{}", c.reason);
    assert!(c.reason.contains(what), "{what} not named in: {}", c.reason);
}

#[test]
fn the_bytes_as_sealed_pass() {
    assert_eq!(sealed_of(&Release::dev()).status, Status::Pass);
}

#[test]
fn one_flipped_byte_anywhere_fails_and_is_named() {
    for f in ["nonos.img", "kernel.bin", "esp/EFI/BOOT/BOOTX64.EFI", "esp/kernel.bin"] {
        let r = Release::dev();
        r.flip(f);
        fails_naming(&r, f);
    }
}

#[test]
fn a_file_missing_from_the_esp_or_one_added_to_it_fails() {
    let r = Release::dev();
    fs::remove_file(r.path().join("esp/kernel.bin")).unwrap();
    fails_naming(&r, "esp/kernel.bin is missing");
    r.put("esp/added.efi", b"x");
    fails_naming(&r, "esp/added.efi is not in the record");
    r.put("esp/kernel.bin", b"the signed kernel");
    fails_naming(&r, "more files than the record names");
}

#[test]
fn with_no_record_the_compare_says_it_did_not_run() {
    let mut ctx = Ctx::new(tempfile::tempdir().unwrap().path().into(), None);
    assert_eq!(sealed::check(&mut ctx).status, Status::NotRun);
}
