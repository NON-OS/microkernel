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

//! The verify matrix's rows and the build-input check: a row passes when it
//! ran steps and each exited 0, and fails naming the one that did not; a
//! matrix that broke, or a row with no steps, does not pass; the build-input
//! check passes only on inputs.py's success.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::check::Status;
use crate::tree::summary::{judge_row, steps};
use crate::tree::{inputs, matrix};

#[test]
fn a_row_whose_steps_pass_passes_and_one_that_fails_is_named() {
    let c = matrix::proof_crates(&mut Sealed::dev().ctx(Fake::default()));
    assert_eq!(c.status, Status::Pass, "{c:?}");
    let c = matrix::static_checks(&mut Sealed::dev().ctx(Fake { step_rc: 1, ..Fake::default() }));
    assert!(
        c.status == Status::Fail && c.reason.contains("`cargo test --release` exited 1"),
        "{c:?}"
    );
}

#[test]
fn a_broken_matrix_or_an_empty_row_does_not_pass() {
    let one = [("cargo test".to_string(), 0)];
    assert_eq!(
        judge_row("matrix.proof-crates", "proof-crates", &one, Some(1)).status,
        Status::Fail
    );
    assert_eq!(
        judge_row("matrix.proof-crates", "proof-crates", &[], Some(0)).status,
        Status::NotRun
    );
    for bad in ["{}", r#"{"rows": {"proof-crates": {"steps": [{"cmd": "x"}]}}}"#, "nope"] {
        assert!(steps(bad.as_bytes(), "proof-crates").is_err(), "{bad}");
    }
}

#[test]
fn the_build_inputs_pass_only_when_inputs_py_does() {
    assert_eq!(inputs::check(&mut Sealed::dev().ctx(Fake::default())).status, Status::Pass);
    let c = inputs::check(&mut Sealed::dev().ctx(Fake { inputs: 1, ..Fake::default() }));
    assert_eq!(c.status, Status::Fail);
}
