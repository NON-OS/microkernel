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

//! FILES_HOME: /home and /root are the family's own only when the policy
//! grants it; the other private prefixes always are, and nothing that
//! merely starts with a prefix's letters is.

use crate::private_rule::{private_for, PRIVATE};

#[test]
fn a_home_is_the_familys_own_only_with_files_home() {
    for path in [&b"/home"[..], b"/home/app/notes", b"/root", b"/root/.cache/x"] {
        assert!(private_for(path, true), "{:?}", core::str::from_utf8(path));
        assert!(!private_for(path, false), "{:?}", core::str::from_utf8(path));
    }
}

#[test]
fn scratch_prefixes_are_private_whatever_the_policy() {
    for p in PRIVATE.iter().filter(|p| !matches!(**p, b"/home" | b"/root")) {
        let mut below = p.to_vec();
        below.extend_from_slice(b"/x");
        for home in [false, true] {
            assert!(private_for(p, home) && private_for(&below, home));
        }
    }
}

#[test]
fn a_name_that_only_starts_like_a_prefix_is_shared() {
    for path in [&b"/homes"[..], b"/rooted/x", b"/tmpfile", b"/run2", b"/", b"/usr/bin/tmp"] {
        assert!(!private_for(path, true), "{:?}", core::str::from_utf8(path));
    }
}
