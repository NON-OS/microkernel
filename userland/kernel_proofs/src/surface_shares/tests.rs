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

use super::rule::{Named, Shares, NAMED_MAX};

const OWNER: u32 = 40;
const VIEWER: u32 = 50;
const CODEC: u32 = 60;
const COMPOSITOR: u32 = 70;

#[test]
fn an_unshared_surface_is_the_owners_alone() {
    let s = Shares::new(OWNER);
    assert!(s.may_attach(OWNER));
    assert!(!s.may_attach(CODEC));
    assert!(!s.may_attach(COMPOSITOR), "not shared yet");
}

#[test]
fn a_guessed_handle_maps_nothing_for_anyone_else() {
    // The finding (G-25): any holder of GraphicsSurfaceMap attached any
    // live handle. Shared with the codec, it stays the codec's.
    let mut s = Shares::new(VIEWER);
    assert_eq!(s.name(CODEC), Named::Added);
    assert!(s.may_attach(CODEC));
    assert!(!s.may_attach(OWNER));
    assert!(!s.may_attach(COMPOSITOR), "named, not the compositor's");
}

#[test]
fn the_compositor_attaches_only_when_named() {
    // No share stands for whoever serves the compositor (K20): it is named
    // as any receiver is, and a restart in a new group is not that group.
    let mut s = Shares::new(OWNER);
    assert_eq!(s.name(COMPOSITOR), Named::Added);
    assert!(s.may_attach(COMPOSITOR));
    assert!(!s.may_attach(71), "a restarted compositor in a new group");
}

#[test]
fn naming_is_idempotent_and_bounded() {
    let mut s = Shares::new(OWNER);
    assert_eq!(s.name(OWNER), Named::Already, "the owner needs no naming");
    assert_eq!(s.name(CODEC), Named::Added);
    assert_eq!(s.name(CODEC), Named::Already);
    for g in 1..NAMED_MAX as u32 {
        assert_eq!(s.name(100 + g), Named::Added);
    }
    assert_eq!(s.name(200), Named::Full);
    assert!(!s.may_attach(200));
    assert!(s.may_attach(CODEC));
}

#[test]
fn group_zero_is_no_process() {
    let mut s = Shares::new(OWNER);
    assert_eq!(s.name(0), Named::Refused);
    assert!(!s.may_attach(0), "an empty entry never matches");
    assert!(!Shares::new(0).may_attach(0));
}
