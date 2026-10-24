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

//! The tree's verifiers are judged by what they report, not by how they exit:
//! a failed check fails; one that could not run does not pass, though
//! nonos-verify exits 0 for it; a report with no checks, or none at all, does
//! not pass; and the gaps a module states become the verdict's limits.

use super::fake::{report, Fake};
use super::sealed_fixture::Sealed;
use crate::check::{Check, Status};
use crate::tree::judge::judge;
use crate::tree::report::parse;
use crate::tree::verify;

pub fn trust_chain(fake: Fake) -> Check {
    verify::trust_chain(&mut Sealed::dev().ctx(fake))
}

#[test]
fn every_check_passing_passes_and_the_stated_gaps_become_limits() {
    let fake = Fake {
        report: |_| Some(report(&[("a", "pass")], &[("field asserts", "exposed fields")])),
        ..Fake::default()
    };
    let c = trust_chain(fake);
    assert_eq!(c.status, Status::Pass, "{c:?}");
    assert_eq!(c.limits, ["not covered: field asserts (it needs: exposed fields)"]);
}

#[test]
fn a_failed_check_or_one_of_unknown_status_fails() {
    let c = trust_chain(Fake {
        report: |_| Some(report(&[("a", "pass"), ("b", "fail")], &[])),
        ..Fake::default()
    });
    assert!(c.status == Status::Fail && c.reason.contains("b (fail)"), "{c:?}");
    let c =
        trust_chain(Fake { report: |_| Some(report(&[("a", "passed")], &[])), ..Fake::default() });
    assert_eq!(c.status, Status::Fail, "an unknown status is no pass");
}

#[test]
fn a_check_that_could_not_run_does_not_pass_though_the_tool_exits_0() {
    for unrun in ["gap", "skip"] {
        let r = report(&[("a", "pass"), ("cargo-audit", unrun)], &[]);
        let c = judge(
            "verify.supply-chain",
            "supply-chain",
            &parse(r.to_string().as_bytes()).unwrap(),
            Some(0),
            &vec![],
        );
        assert!(c.status == Status::NotRun && c.reason.contains("cargo-audit"), "{c:?}");
    }
}
