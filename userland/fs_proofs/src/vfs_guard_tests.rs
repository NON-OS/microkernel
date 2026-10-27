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

//! A Linux app's home and private files are reached only by a caller
//! holding ForeignExec, and shown to nobody in a listing, a search or the
//! recent-files journal (capsule_vfs/src/server/fs_gate/guard.rs). The
//! shipping rules, included by path, with the kernel played by a closure.

use super::guard::guarded;
use super::reach::reaches;
use crate::protocol::{OP_OPEN, OP_READ, OP_STAT};
use std::cell::Cell;

pub(super) fn pair(path: &str) -> Vec<u8> {
    let mut out = vec![path.len() as u8];
    out.extend_from_slice(path.as_bytes());
    out
}

pub(super) fn asks(op: u16, rest: &[u8], foreign: bool) -> (bool, bool) {
    let asked = Cell::new(false);
    let served = reaches(op, rest, crate::normalize, || {
        asked.set(true);
        foreign
    });
    (served, asked.get())
}

#[test]
fn a_tree_is_guarded_by_whole_components() {
    for p in ["/linux-home", "/linux-home/ab", "/linux-private/01/root/k", "/linux-home/"] {
        assert!(guarded(p), "{p}");
    }
    for p in ["/linux-homes", "/linux-home2/x", "/linux", "/", "/home/linux-home", "/linux-pr"] {
        assert!(!guarded(p), "{p}");
    }
}

#[test]
fn a_guarded_path_needs_foreign_exec_and_others_never_ask() {
    let home = pair("/linux-home/0f/root/wallet.keys");
    assert_eq!(asks(OP_OPEN, &home, false), (false, true));
    assert_eq!(asks(OP_OPEN, &home, true), (true, true));
    assert_eq!(asks(OP_STAT, &pair("/nonos/setup/answers"), false), (true, false));
    assert_eq!(asks(OP_READ, &[0u8; 8], false), (true, false), "an fd op names no path");
}

#[test]
fn a_spelling_that_normalizes_into_a_tree_is_guarded() {
    for p in ["//linux-private//1/y", "/linux-home/./0f", "/./linux-home"] {
        assert!(!asks(OP_STAT, &pair(p), false).0, "{p}");
    }
    // A `..` never normalizes, so its handler refuses it before the store.
    for p in ["/tmp/../linux-home/0f/x", "/linux-home/../tmp/x"] {
        assert_eq!(crate::normalize(p), None, "{p}");
    }
}
