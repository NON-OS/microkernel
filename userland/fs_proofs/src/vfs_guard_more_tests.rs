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

//! The guard on requests that name two paths or act on a subtree, and on
//! listings (capsule_vfs/src/server/fs_gate/guard.rs).

use super::guard::{strip_listing, touches};
use super::guard_tests::{asks, pair};
use crate::protocol::{OP_COPY, OP_LIST, OP_RENAME, OP_RMDIR};

#[test]
fn both_paths_of_a_rename_or_copy_are_asked() {
    let mut out_of = pair("/linux-home/0f/k");
    out_of.extend(pair("/tmp/k"));
    let mut into = pair("/tmp/k");
    into.extend(pair("/linux-private/1/k"));
    for rest in [&out_of, &into] {
        assert!(!asks(OP_RENAME, rest, false).0);
        assert!(!asks(OP_COPY, rest, false).0);
    }
}

#[test]
fn a_subtree_op_on_the_root_is_held_off_the_trees() {
    let mut root = pair("/");
    root.extend(pair("/backup"));
    assert!(touches("/") && !touches("/tmp") && !touches("/linux"));
    assert!(!asks(OP_COPY, &[root.as_slice(), &[1]].concat(), false).0);
    assert!(!asks(OP_RMDIR, &pair("/"), false).0);
    assert!(asks(OP_LIST, &pair("/"), false).0, "a listing is filtered instead");
}

#[test]
fn a_listing_leaves_the_trees_out() {
    let body: Vec<u8> = ["/linux-home/", "/notes.txt", "/linux-private/9/x", "/linux-homes"]
        .iter()
        .flat_map(|n| pair(n))
        .collect();
    let want: Vec<u8> = ["/notes.txt", "/linux-homes"].iter().flat_map(|n| pair(n)).collect();
    assert_eq!(strip_listing(&body), want);
    assert_eq!(strip_listing(&body[..3]), Vec::<u8>::new(), "a torn tail is dropped");
}
