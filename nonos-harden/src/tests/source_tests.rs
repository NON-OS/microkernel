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

//! `tree.source`: the tree is the release's source: its commit, or a later
//! one changed only in the trust set the seal stages, with nothing else
//! changed or added. Anything else is named.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::check::{Check, Status};
use crate::release::{load, receipt_check};
use crate::tree::{ledger::TRUST, source};

fn source_of(s: &Sealed) -> Check {
    let mut ctx = s.ctx(Fake::default());
    load::check(&mut ctx);
    receipt_check::check(&mut ctx);
    source::check(&mut ctx)
}

pub fn passes(s: &Sealed, why: &str) {
    let c = source_of(s);
    assert!(c.status == Status::Pass && c.reason.contains(why), "{c:?}");
}

pub fn fails(s: &Sealed, why: &str) {
    let c = source_of(s);
    assert!(c.status == Status::Fail && c.reason.contains(why), "{why:?}: {c:?}");
}

#[test]
fn the_tree_at_the_receipts_commit_is_the_source() {
    passes(&Sealed::dev(), "exactly commit");
}

#[test]
fn the_trust_set_staged_or_committed_after_is_the_one_change_allowed() {
    let s = Sealed::dev();
    s.tree.put(&format!("{TRUST}/capsules/added.manifest.bin"), b"staged by the seal");
    s.tree.ledger();
    passes(&s, "changed only where the seal stages");
    s.tree.git(&["add", "-A"]);
    passes(&s, "changed only where the seal stages");
    s.tree.git(&["commit", "-q", "-m", "the trust set the seal staged"]);
    passes(&s, "changed only where the seal stages");
}

#[test]
fn a_change_or_an_addition_outside_the_trust_set_fails_and_is_named() {
    let s = Sealed::dev();
    s.tree.put("src/main.rs", b"fn main() { /* not what was built */ }\n");
    fails(&s, "src/main.rs");
    let s = Sealed::dev();
    s.tree.put("scripts/check_added.py", b"print('a check nobody built with')\n");
    fails(&s, "scripts/check_added.py");
}
