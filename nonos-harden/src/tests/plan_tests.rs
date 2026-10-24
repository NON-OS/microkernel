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

//! The whole command runs every check in order every time, and every verifier
//! of the tree.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::ctx::Ctx;
use crate::exit;
use crate::plan::{run, STEPS};

/// Written out, not read from the plan, so a check dropped from it fails here.
pub const EXPECTED: &[&str] = &[
    "release.record",
    "release.sealed",
    "release.kind",
    "release.receipt",
    "tree.source",
    "tree.ledger",
    "release.kernel-signature",
    "tree.build-inputs",
    "verify.hygiene",
    "verify.trust-chain",
    "verify.supply-chain",
    "matrix.static-checks",
    "matrix.proof-crates",
];

#[test]
fn every_check_runs_in_order_every_time() {
    let s = Sealed::dev();
    let ids = |mut ctx: Ctx| run(&mut ctx).iter().map(|c| c.id).collect::<Vec<_>>();
    assert_eq!(ids(s.ctx(Fake::default())), EXPECTED);
    assert_eq!(ids(Ctx::new(tempfile::tempdir().unwrap().path().into(), None)), EXPECTED);
    for (id, step) in STEPS {
        assert_eq!(step(&mut s.ctx(Fake::default())).id, *id);
    }
}

#[test]
fn every_verifier_of_the_tree_is_run() {
    let fake = Fake::default();
    let ran = fake.ran.clone();
    assert_eq!(Sealed::dev().verdict(fake).0, exit::DEV_GO);
    let want = ["python3 --check", "cargo hygiene", "cargo trust-chain", "cargo supply-chain"];
    let want = [&want[..], &["python3 static-checks", "python3 proof-crates"]].concat();
    assert_eq!(*ran.borrow(), want);
}
