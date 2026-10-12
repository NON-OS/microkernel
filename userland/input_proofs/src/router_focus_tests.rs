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

//! Focus-only input: a grab only for a named capsule that has focus (the
//! shell when the desktop has it, the name alone when no window manager
//! answers), and keys after the focused window closes going to the shell,
//! never to the closed window, while the release of a key it held still
//! reaches it. A held key repeating into a new focus is proved in
//! router_key_tests.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::route::grab_rule::{grab_allowed, may_grab};
use crate::router_desk::*;

const A: u32 = 20;
const B: u32 = 21;

#[test]
fn a_grab_from_a_client_without_focus_is_refused() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    window(&mut ctx, B, (400, 0, 300, 300));
    world::set_focus(A);
    assert!(!grab_allowed(&mut ctx, B, true), "named, but A has focus");
    assert!(grab_allowed(&mut ctx, A, true));
    assert!(!grab_allowed(&mut ctx, A, false), "focused, but not named");
    world::close(A);
    world::close(B);
    assert!(grab_allowed(&mut ctx, SHELL, true), "the desktop has focus");
    assert!(!grab_allowed(&mut ctx, A, true));
}

#[test]
fn with_no_window_manager_the_name_alone_decides() {
    assert!(may_grab(B, true, None, SHELL), "setup and the installer");
    assert!(!may_grab(B, false, None, SHELL));
    assert!(!may_grab(0, true, None, SHELL));
}

#[test]
fn keys_after_the_focused_window_closes_go_to_the_shell() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    key_down(&mut ctx, u32::from(b'x'), 0);
    world::close(A);
    key_up(&mut ctx, u32::from(b'x'), 0);
    key_down(&mut ctx, u32::from(b'y'), 0);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(A, u32::from(b'x'))], "its own release");
    let downs = of_kind(&rx, INPUT_KIND_KEY_DOWN);
    assert_eq!(downs, [(A, u32::from(b'x')), (SHELL, u32::from(b'y'))]);
}
