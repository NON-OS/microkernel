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

//! An unauthenticated keyboard, bonded Just Works, is recorded and resumed
//! as unauthenticated — the level an input router reads to keep it off the
//! secure-attention path. The authenticated keyboard is `bond_tests`; this
//! is its mirror, so the two levels are told apart end to end.

use crate::bond_tests::{press, static_identity};
use crate::fixture::{air, link, unauth_keyboard};
use crate::link::End;

#[test]
fn a_just_works_keyboard_is_bonded_unauthenticated() {
    let (mut host, ours, mut env) = unauth_keyboard();
    static_identity(&mut host);
    press(&mut host);
    let (_, s) = link(&mut host, &ours, &mut env).unwrap();
    assert!(!s.bond.unwrap().authenticated, "Just Works is not authenticated");
    let kept = env.bonds.book().bonds();
    assert_eq!(kept.len(), 1);
    assert!(!kept[0].authenticated, "the kept record carries the unauthenticated level");
    // No passkey was ever shown: no user step protected this pairing.
    assert!(env.passkeys.is_empty(), "Just Works asks the user nothing");
}

#[test]
fn it_resumes_at_the_same_unauthenticated_level() {
    let (mut host, ours, mut env) = unauth_keyboard();
    static_identity(&mut host);
    press(&mut host);
    link(&mut host, &ours, &mut env).unwrap();
    press(&mut host);
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert_eq!(air(&mut host).seen.requests, 1, "a resume never pairs again");
    assert!(env.said("encrypted with the kept bond"), "{:?}", env.lines);
    assert!(!s.bond.unwrap().authenticated, "it resumes unauthenticated, never silently raised");
}
