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
//! names the one check that caught it and the file it caught it in.

use super::fixture::Release;
use super::plan_tests::verdict;
use crate::exit;
use std::path::Path;
use std::process::Command;

const FLIP: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../tools/nonos-flip-byte");

#[test]
fn a_copy_with_one_flipped_byte_reads_no_go_naming_the_check() {
    let r = Release::dev();
    let copy = tempfile::tempdir().unwrap();
    let src = format!("{}/.", r.path().display());
    assert!(Command::new("cp").args(["-a", &src]).arg(copy.path()).status().unwrap().success());
    let img = |d: &Path| d.join("nonos.img");
    let flip = Command::new("python3").arg(FLIP).arg(img(r.path())).arg(img(copy.path())).status();
    assert!(flip.unwrap().success());
    let (code, v) = verdict(copy.path());
    assert_eq!(code, exit::NO_GO);
    let checks = v["checks"].as_array().unwrap();
    let caught: Vec<_> = checks.iter().filter(|c| c["status"] != "pass").collect();
    assert_eq!(caught.len(), 1, "exactly one check caught it");
    assert_eq!(caught[0]["id"], "release.sealed");
    assert!(caught[0]["reason"].as_str().unwrap().contains("nonos.img"));
    assert!(v["verdict"].as_str().unwrap().ends_with("first release.sealed"));
    assert_eq!(verdict(r.path()).0, exit::GO, "and the release it was copied from still passes");
}
