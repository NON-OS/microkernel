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

//! A verifier that left no report, an empty one, or that exited badly with
//! every check passing does not pass; a report of another shape is refused.

use super::fake::{report, Fake};
use super::judge_tests::trust_chain;
use crate::check::Status;
use crate::tree::judge::judge;
use crate::tree::report::parse;

#[test]
fn no_report_an_empty_one_or_a_bad_exit_does_not_pass() {
    assert_eq!(trust_chain(Fake { report: |_| None, ..Fake::default() }).status, Status::NotRun);
    assert_eq!(
        trust_chain(Fake { report: |_| Some(report(&[], &[])), ..Fake::default() }).status,
        Status::NotRun
    );
    let r = parse(report(&[("a", "pass")], &[]).to_string().as_bytes()).unwrap();
    assert_eq!(judge("verify.hygiene", "hygiene", &r, Some(2), &vec![]).status, Status::Fail);
    assert_eq!(judge("verify.hygiene", "hygiene", &r, None, &vec![]).status, Status::Fail);
}

#[test]
fn a_report_in_another_shape_is_refused() {
    for bad in [
        "[]",
        "{}",
        r#"{"checks": [{"name": "a"}], "gaps": []}"#,
        r#"{"checks": [], "gaps": [{"what": 1}]}"#,
    ] {
        assert!(parse(bad.as_bytes()).is_err(), "{bad}");
    }
}
