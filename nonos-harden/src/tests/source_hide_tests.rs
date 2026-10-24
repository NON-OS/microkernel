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

//! Nothing in git's view of the tree can hide a change from `tree.source`,
//! because it hashes the files itself: a file git ignores, a rename into
//! the staged trust set, an index flag, a clean filter, an executable bit,
//! or a link standing in for a file are each a change, and each is named.

use super::sealed_fixture::Sealed;
use super::source_tests::fails;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};

const MAIN: &str = "src/main.rs";

#[test]
fn a_file_git_ignores_is_still_a_file_the_commit_does_not_hold() {
    let s = Sealed::dev();
    fs::write(s.tree.path().join(".git/info/exclude"), "*.pyc\n").unwrap();
    s.tree.put("tools/json.pyc", b"bytecode that shadows a module");
    fails(&s, "tools/json.pyc");
}

#[test]
fn a_rename_into_the_trust_set_is_a_file_gone_from_where_it_was() {
    let s = Sealed::dev();
    s.tree.git(&["mv", MAIN, "nonos-data/trust/main.rs"]);
    fails(&s, MAIN);
}

#[test]
fn an_edit_behind_an_index_flag_or_a_clean_filter_is_an_edit() {
    let s = Sealed::dev();
    s.tree.git(&["update-index", "--skip-worktree", MAIN]);
    s.tree.put(MAIN, b"fn main() { /* hidden from git status */ }\n");
    fails(&s, MAIN);
    let s = Sealed::dev();
    s.tree.git(&["config", "filter.hide.clean", "printf 'fn main() {}\\n'"]);
    fs::write(s.tree.path().join(".git/info/attributes"), "src/main.rs filter=hide\n").unwrap();
    s.tree.put(MAIN, b"fn main() { /* what the filter would hide */ }\n");
    fails(&s, MAIN);
}

#[test]
fn a_changed_executable_bit_or_a_link_in_place_of_a_file_is_a_change() {
    let s = Sealed::dev();
    let at = s.tree.path().join(MAIN);
    fs::set_permissions(&at, fs::Permissions::from_mode(0o755)).unwrap();
    fails(&s, MAIN);
    let s = Sealed::dev();
    let (at, copy) = (s.tree.path().join(MAIN), s.tree.path().join("nonos-data/trust/same.rs"));
    fs::copy(&at, &copy).unwrap();
    fs::remove_file(&at).unwrap();
    symlink(&copy, &at).unwrap();
    fails(&s, MAIN);
}
