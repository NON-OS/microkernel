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

//! Any capsule can send to the shell's inbox; a power key or the process
//! manager chord from one that is not the input router used to be obeyed.

use crate::shell_peer_rule::from_peer;

const ROUTER: u32 = 41;

#[test]
fn the_router_known_by_pid_is_believed_without_asking_the_registry() {
    let asked = core::cell::Cell::new(false);
    assert_eq!(
        from_peer(ROUTER, ROUTER, || {
            asked.set(true);
            0
        }),
        (true, ROUTER)
    );
    assert!(!asked.get());
}

#[test]
fn any_other_sender_is_refused_even_while_the_router_is_known() {
    for sender in [1, 7, ROUTER + 1, u32::MAX] {
        assert_eq!(from_peer(sender, ROUTER, || ROUTER), (false, ROUTER), "{sender}");
    }
}

#[test]
fn nobody_is_never_the_router_even_when_none_is_registered() {
    assert_eq!(from_peer(0, 0, || 0), (false, 0));
    assert_eq!(from_peer(0, ROUTER, || 0), (false, ROUTER));
}

#[test]
fn a_router_restarted_under_a_new_pid_is_followed() {
    assert_eq!(from_peer(77, ROUTER, || 77), (true, 77));
    assert_eq!(from_peer(ROUTER, 77, || 77), (false, 77), "the old pid is not believed");
}

#[test]
fn with_no_router_registered_nobody_is_believed() {
    assert_eq!(from_peer(ROUTER, 0, || 0), (false, 0));
}
