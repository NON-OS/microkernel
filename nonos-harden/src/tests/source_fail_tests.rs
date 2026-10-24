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

//! A tree that does not hold the release's commit, whose HEAD does not come
//! after it, or that is not a git checkout at all, is not the release's source.

use super::receipt_tests::edit;
use super::sealed_fixture::Sealed;
use super::source_tests::fails;

#[test]
fn a_commit_the_tree_lacks_or_a_head_before_it_fails() {
    let s = Sealed::dev();
    edit(&s, |v| v["source"]["commit"] = "0".repeat(40).into());
    fails(&s, "does not hold commit");
    let s = Sealed::dev();
    s.tree.git(&["checkout", "-q", "--orphan", "elsewhere"]);
    s.tree.git(&["commit", "-q", "-m", "unrelated history"]);
    fails(&s, "neither commit");
}

#[test]
fn a_directory_that_is_not_a_checkout_fails() {
    let s = Sealed::dev();
    std::fs::remove_dir_all(s.tree.path().join(".git")).unwrap();
    fails(&s, "not a git checkout");
}
