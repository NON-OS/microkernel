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

//! A copy of a passing release with one byte flipped by the tree's own
//! negative-test tool (tools/nonos-flip-byte) reads NO-GO, and the verdict
//! names every check that caught it and the file it caught it in.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::exit;
use std::process::Command;

const FLIP: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../tools/nonos-flip-byte");

/// The ids of the checks that did not pass when `file` is flipped in a copy.
fn caught(file: &str) -> Vec<String> {
    let s = Sealed::dev();
    let copy = tempfile::tempdir().unwrap();
    let src = format!("{}/.", s.release.path().display());
    assert!(Command::new("cp").args(["-a", &src]).arg(copy.path()).status().unwrap().success());
    let mut flip = Command::new("python3");
    flip.arg(FLIP).arg(s.release.path().join(file)).arg(copy.path().join(file));
    assert!(flip.status().unwrap().success());
    let (code, v) = s.verdict_of(copy.path(), Fake::default());
    assert_eq!(code, exit::NO_GO);
    let missed: Vec<_> =
        v["checks"].as_array().unwrap().iter().filter(|c| c["status"] != "pass").collect();
    assert!(missed.iter().all(|c| c["reason"].as_str().unwrap().contains(file)), "{missed:#?}");
    assert_eq!(
        s.verdict(Fake::default()).0,
        exit::DEV_GO,
        "the release it was copied from still passes"
    );
    missed.iter().map(|c| c["id"].as_str().unwrap().to_string()).collect()
}

#[test]
fn a_flipped_byte_in_the_image_reads_no_go_naming_the_check() {
    assert_eq!(caught("nonos.img"), ["release.sealed"]);
}

#[test]
fn a_flipped_byte_in_the_kernel_is_caught_by_its_digest_and_its_signature() {
    assert_eq!(caught("kernel.bin"), ["release.sealed", "release.kernel-signature"]);
}
