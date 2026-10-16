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

//! Input is believed from the router the shell first found, and from no
//! later holder of its name: a window posing as it is refused, and once that
//! router is gone nothing is believed, whoever registers the name next.

use crate::state::input_source_rule::Router;

#[test]
fn the_first_holder_is_pinned_and_believed_alone() {
    let r = Router::Unknown.after_lookup(Some(12));
    assert_eq!(r, Router::Pinned(12));
    assert!(r.believes(12));
    assert!(!r.believes(13), "a window posing as the router");
    assert!(!r.believes(0));
    assert_eq!(r.after_lookup(Some(12)), r, "asked again while it runs");
}

#[test]
fn nothing_is_believed_before_a_router_runs() {
    let r = Router::Unknown.after_lookup(None);
    assert_eq!(r, Router::Unknown);
    assert!(!r.believes(0) && !r.believes(12));
    assert_eq!(Router::Unknown.after_lookup(Some(0)), Router::Unknown);
    assert_eq!(Router::Unknown.after_lookup(Some(u32::MAX)), Router::Unknown);
}

#[test]
fn once_the_router_is_gone_no_new_holder_is_believed() {
    let pinned = Router::Pinned(12);
    for gone in [pinned.after_lookup(None), pinned.after_lookup(Some(40))] {
        assert_eq!(gone, Router::Lost);
        assert!(!gone.believes(12) && !gone.believes(40));
        assert_eq!(gone.after_lookup(Some(40)), Router::Lost, "a later holder proves nothing");
        assert_eq!(gone.after_lookup(Some(12)), Router::Lost);
    }
}

#[test]
fn the_rule_survives_its_one_word() {
    for r in [Router::Unknown, Router::Pinned(1), Router::Pinned(u32::MAX - 1), Router::Lost] {
        assert_eq!(Router::from_word(r.word()), r);
    }
}
