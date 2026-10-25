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
    assert!(s.may_attach(OWNER, Some(COMPOSITOR)));
    assert!(!s.may_attach(CODEC, Some(COMPOSITOR)));
    assert!(!s.may_attach(COMPOSITOR, Some(COMPOSITOR)), "not shared yet");
}

#[test]
fn a_guessed_handle_maps_nothing_for_anyone_else() {
    // The finding (G-25): any holder of GraphicsSurfaceMap attached any
    // live handle. Shared with the codec, it stays the codec's.
    let mut s = Shares::new(VIEWER);
    assert_eq!(s.name(CODEC), Named::Added);
    assert!(s.may_attach(CODEC, Some(COMPOSITOR)));
    assert!(!s.may_attach(OWNER, Some(COMPOSITOR)));
    assert!(!s.may_attach(COMPOSITOR, Some(COMPOSITOR)), "named, not the compositor's");
}

#[test]
fn the_older_share_is_for_whoever_serves_the_compositor() {
    let mut s = Shares::new(OWNER);
    s.allow_compositor();
    assert!(s.may_attach(COMPOSITOR, Some(COMPOSITOR)));
    // A restarted compositor in a new group attaches; the old one does not.
    assert!(s.may_attach(71, Some(71)));
    assert!(!s.may_attach(COMPOSITOR, Some(71)));
    assert!(!s.may_attach(COMPOSITOR, None), "no compositor, no attach");
    assert!(!s.may_attach(CODEC, Some(COMPOSITOR)));
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
    assert!(!s.may_attach(200, None));
    assert!(s.may_attach(CODEC, None));
}

#[test]
fn group_zero_is_no_process() {
    let mut s = Shares::new(OWNER);
    assert_eq!(s.name(0), Named::Refused);
    s.allow_compositor();
    assert!(!s.may_attach(0, Some(0)), "an empty entry never matches");
    assert!(!Shares::new(0).may_attach(0, None));
}
