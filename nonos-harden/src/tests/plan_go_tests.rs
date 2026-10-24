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

//! GO, NO-GO and an undelivered verdict each come out as the interface says:
//! a passing development release reads GO, says it is not a release and exits
//! 5, never the 0 a release's GO exits with; with no tree every tree check
//! does not run, which is NO-GO.

use super::fake::Fake;
use super::plan_tests::EXPECTED;
use super::sealed_fixture::Sealed;
use crate::check::Status;
use crate::cli::CheckArgs;
use crate::ctx::{Ctx, Kind};
use crate::exit;
use crate::plan::{check_with, run};
use crate::verdict::{decide, Overall, DEV_GO};

#[test]
fn a_passing_development_release_reads_go_not_a_release_and_a_release_go() {
    // Each GO carries release.sealed's one limit, so each line counts it.
    let (code, v) = Sealed::dev().verdict(Fake::default());
    let dev = format!("{DEV_GO}, 1 declared limit");
    let got = (code, v["verdict"].as_str(), v["kind"].as_str());
    assert_eq!(got, (exit::DEV_GO, Some(&*dev), Some("development")), "{v:#}");
    let (code, v) = Sealed::release().verdict(Fake::default());
    assert_eq!((code, v["verdict"].as_str()), (exit::GO, Some("GO, 1 declared limit")), "{v:#}");
}

#[test]
fn without_a_tree_every_tree_check_does_not_run_and_it_is_no_go() {
    let s = Sealed::dev();
    let checks =
        run(&mut Ctx::with_runner(s.release.path().into(), None, Box::new(Fake::default())));
    let unrun: Vec<_> =
        checks.iter().filter(|c| c.status == Status::NotRun).map(|c| c.id).collect();
    assert_eq!(unrun, EXPECTED[4..], "every check from tree.source on needs the tree");
    assert_eq!(decide(Kind::Development, &checks).0, Overall::NoGo);
}

#[test]
fn a_verdict_that_cannot_be_written_is_not_delivered() {
    let s = Sealed::dev();
    let out = Some(s.release.path().join("no/such/dir/verdict.json"));
    let args = CheckArgs {
        release: s.release.path().into(),
        tree: Some(s.tree.path().into()),
        out,
        sign_key: None,
        json: false,
    };
    assert_eq!(check_with(&args, Box::new(Fake::default())), exit::NOT_DELIVERED);
}
