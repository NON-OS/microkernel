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

//! The whole command: every check runs in order every time; GO, NO-GO and an
//! undelivered verdict each come out as the interface says.

use super::fixture::Release;
use crate::cli::CheckArgs;
use crate::ctx::Ctx;
use crate::exit;
use crate::plan::{check, run, STEPS};
use crate::verdict::DEV_GO;
use std::path::Path;

/// Written out, not read from the plan, so a check dropped from it fails here.
const EXPECTED: &[&str] = &["release.record", "release.sealed", "release.kind"];

pub fn verdict(release: &Path) -> (u8, serde_json::Value) {
    let out = tempfile::tempdir().unwrap();
    let file = out.path().join("verdict.json");
    let args = CheckArgs { release: release.into(), tree: None, out: Some(file.clone()), json: true };
    let code = check(&args);
    (code, serde_json::from_slice(&std::fs::read(file).unwrap()).unwrap())
}

#[test]
fn every_check_runs_in_order_every_time() {
    for dir in [Release::dev().dir, tempfile::tempdir().unwrap()] {
        let ids: Vec<_> = run(&mut Ctx::new(dir.path().into(), None)).iter().map(|c| c.id).collect();
        assert_eq!(ids, EXPECTED);
    }
    for (id, step) in STEPS {
        assert_eq!(step(&mut Ctx::new(Release::dev().path().into(), None)).id, *id);
    }
}

#[test]
fn a_passing_development_release_reads_go_not_a_release() {
    let (code, v) = verdict(Release::dev().path());
    assert_eq!((code, v["verdict"].as_str()), (exit::GO, Some(DEV_GO)));
    assert_eq!(v["kind"], "development");
}

#[test]
fn a_passing_release_reads_go() {
    let (code, v) = verdict(Release::release().path());
    assert_eq!((code, v["verdict"].as_str(), v["kind"].as_str()), (exit::GO, Some("GO"), Some("release")));
}

#[test]
fn a_directory_that_is_not_a_release_is_no_go() {
    let (code, v) = verdict(tempfile::tempdir().unwrap().path());
    assert_eq!((code, v["checks"][0]["status"].as_str()), (exit::NO_GO, Some("fail")));
}

#[test]
fn a_verdict_that_cannot_be_written_is_not_delivered() {
    let r = Release::dev();
    let out = Some(r.path().join("no/such/dir/verdict.json"));
    let args = CheckArgs { release: r.path().into(), tree: None, out, json: false };
    assert_eq!(check(&args), exit::NOT_DELIVERED);
}
