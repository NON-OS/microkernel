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

use crate::{is_read_only, normalize};

#[test]
fn collapses_duplicate_slashes() {
    assert_eq!(normalize("/a//b").as_deref(), Some("/a/b"));
    assert_eq!(normalize("/a///b//c").as_deref(), Some("/a/b/c"));
}

#[test]
fn drops_dot_components() {
    assert_eq!(normalize("/a/./b").as_deref(), Some("/a/b"));
    assert_eq!(normalize("/./a").as_deref(), Some("/a"));
}

#[test]
fn refuses_parent_components() {
    // Callers resolve `..` themselves; vfs never decides what a climb reaches.
    for path in ["/a/../b", "/a/b/c/../../d", "/a/b/..", "/..", "/../..", "..", "/linux/../capsules"] {
        assert_eq!(normalize(path), None, "{path}");
    }
}

#[test]
fn strips_trailing_slash_except_root() {
    assert_eq!(normalize("/a/b/").as_deref(), Some("/a/b"));
    assert_eq!(normalize("/a/").as_deref(), Some("/a"));
    assert_eq!(normalize("/").as_deref(), Some("/"));
}

#[test]
fn empty_and_root_normalize_to_root() {
    assert_eq!(normalize("").as_deref(), Some("/"));
    assert_eq!(normalize("//").as_deref(), Some("/"));
}

#[test]
fn adds_leading_slash_to_relative() {
    assert_eq!(normalize("a/b").as_deref(), Some("/a/b"));
    assert_eq!(normalize("a").as_deref(), Some("/a"));
}

#[test]
fn dots_that_are_names_are_kept() {
    // Only a whole `..` component is refused; names containing dots are not.
    assert_eq!(normalize("/a/..b/c...").as_deref(), Some("/a/..b/c..."));
}

#[test]
fn read_only_covers_capsules_tree_only() {
    assert!(is_read_only("/capsules"));
    assert!(is_read_only("/capsules/std_proof.elf"));
    assert!(is_read_only("/capsules/nested/deep.bin"));
}

#[test]
fn read_only_rejects_lookalikes_and_others() {
    assert!(!is_read_only("/capsulesX"));
    assert!(!is_read_only("/capsules-backup"));
    assert!(!is_read_only("/docs"));
    assert!(!is_read_only("/"));
    assert!(!is_read_only("/capsule"));
}

#[test]
fn normalized_capsules_path_is_still_guarded() {
    // The guard runs on the normalized form, so slash tricks cannot smuggle a
    // write into the protected tree.
    assert!(normalize("/capsules//evil").as_deref().is_some_and(is_read_only));
    // A climb back into the tree is not normalised into it: it is refused.
    assert_eq!(normalize("/capsules/../capsules/evil"), None);
}
