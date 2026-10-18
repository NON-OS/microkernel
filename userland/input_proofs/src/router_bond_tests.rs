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
//! The secure attention chord from a Bluetooth keyboard: refused,
//! press and release, from one paired by Just Works or not stamped with its
//! level, with the shell told; let through from one paired authenticated,
//! and from a wired keyboard as before. Ordinary keys reach windows either
//! way, and no receiver sees the pairing bits.

use nonos_libc::INPUT_KIND_KEY_DOWN;

use crate::clients::world;
use crate::route::bond::{BOND_AUTHENTICATED, BOND_BITS};
use crate::route::chord::KEY_DELETE;
use crate::router_tests::router_bond_kit::*;
use crate::router_desk::*;

const A: u32 = 20;
const KEY_X: u32 = b'x' as u32;

fn focused() -> crate::state::Context {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    ctx
}

#[test]
fn just_works_chord_goes_nowhere_and_the_shell_is_told() {
    let mut ctx = focused();
    chord(&mut ctx, JUST_WORKS);
    let sent = out();
    assert_eq!(events(&sent), [], "a Just Works chord was delivered: {sent:?}");
    assert_eq!(notices(&sent), [SHELL]);
}

#[test]
fn only_an_authenticated_bond_or_a_wire_reaches_the_shell() {
    for bond in [PASSKEY, 0, BOND_AUTHENTICATED] {
        let mut ctx = focused();
        chord(&mut ctx, bond);
        let sent = out();
        assert_eq!(events(&sent), [(SHELL, KEY_DELETE); 2], "bond {bond:#x}: {sent:?}");
        assert_eq!(notices(&sent), [], "bond {bond:#x} was refused");
        assert!(sent.iter().all(|o| o.flags & BOND_BITS == 0), "pairing bits leaked");
    }
}

#[test]
fn ordinary_keys_reach_the_focused_window_without_the_bits() {
    let mut ctx = focused();
    key_down(&mut ctx, KEY_X, JUST_WORKS | MOD_CTRL);
    let sent = out();
    assert_eq!(events(&sent), [(A, KEY_X)]);
    assert_eq!(sent[0].flags, MOD_CTRL, "modifiers kept, pairing bits stripped");
    assert_eq!(sent[0].kind, INPUT_KIND_KEY_DOWN);
}
