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

//! The lid keys, which the kernel posts on the power key's path, go to the
//! desktop shell whatever has focus, down and up, as the other system keys
//! do; the codes are the ones app_skeleton names, beside KEY_POWER.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::route::shell_keys::{is_shell_key, KEY_LID_CLOSED, KEY_LID_OPENED, KEY_POWER};
use crate::router_desk::*;

const A: u32 = 20;

/// app_skeleton's key table, unchanged, for the lid's and brightness's codes.
#[path = "../../app_skeleton/src/input/keys.rs"]
pub mod skeleton;

#[test]
fn the_lid_codes_follow_the_power_key_and_agree_with_app_skeleton() {
    assert_eq!((KEY_LID_CLOSED, KEY_LID_OPENED), (KEY_POWER + 1, KEY_POWER + 2));
    assert_eq!(
        (KEY_LID_CLOSED, KEY_LID_OPENED),
        (skeleton::KEY_LID_CLOSED, skeleton::KEY_LID_OPENED)
    );
    assert!(is_shell_key(KEY_LID_CLOSED) && is_shell_key(KEY_LID_OPENED));
}

#[test]
fn the_lid_goes_to_the_shell_while_a_window_has_focus() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    world::set_focus(A);
    for code in [KEY_LID_OPENED, KEY_LID_CLOSED, KEY_LID_OPENED] {
        key_down(&mut ctx, code, 0);
        key_up(&mut ctx, code, 0);
    }
    let rx = sent();
    let lid = [(SHELL, KEY_LID_OPENED), (SHELL, KEY_LID_CLOSED), (SHELL, KEY_LID_OPENED)];
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), lid);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), lid);
    assert!(rx.iter().all(|r| r.pid != A), "the focused window saw the lid: {rx:?}");
}
