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

//! Input is believed from whoever the registry names as the router, which
//! with L52 is always the image's own: a window posing as it is refused
//! while the router runs, and a restarted router is believed at once, the
//! old pid no longer.

use crate::state::input_source_rule::Router;

#[test]
fn the_named_router_is_believed_alone() {
    let r = Router::Unknown.after_lookup(Some(12));
    assert_eq!(r, Router::Pinned(12));
    assert!(r.believes(12));
    assert!(!r.believes(13), "a window posing as the router");
    assert!(!r.believes(0));
    assert_eq!(r.after_lookup(Some(12)), r, "a poser's frame: asked again, still 12");
}

#[test]
fn nothing_is_believed_while_no_router_runs() {
    let r = Router::Unknown.after_lookup(None);
    assert_eq!(r, Router::Unknown);
    assert!(!r.believes(0) && !r.believes(12));
    assert_eq!(Router::Unknown.after_lookup(Some(0)), Router::Unknown);
    assert_eq!(Router::Pinned(12).after_lookup(None), Router::Unknown, "the router exited");
}

#[test]
fn a_restarted_router_is_believed_and_the_old_pid_is_not() {
    let r = Router::Pinned(12).after_lookup(Some(40));
    assert_eq!(r, Router::Pinned(40));
    assert!(r.believes(40) && !r.believes(12));
}

#[test]
fn the_rule_survives_its_one_word() {
    for r in [Router::Unknown, Router::Pinned(1), Router::Pinned(u32::MAX)] {
        assert_eq!(Router::from_word(r.word()), r);
    }
}
