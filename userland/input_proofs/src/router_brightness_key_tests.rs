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

//! The brightness keys, which the kernel posts on the lid's path from the
//! firmware's brightness notifications, go to the desktop shell whatever
//! has focus, down and up; the codes are the ones app_skeleton names.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::route::shell_keys::{
    is_shell_key, KEY_BRIGHTNESS_DOWN, KEY_BRIGHTNESS_UP, KEY_LID_OPENED,
};
use crate::router_desk::*;
use crate::router_tests::router_lid_key_tests::skeleton;

const A: u32 = 20;

#[test]
fn the_brightness_codes_follow_the_lid_and_agree_with_app_skeleton() {
    assert_eq!((KEY_BRIGHTNESS_UP, KEY_BRIGHTNESS_DOWN), (KEY_LID_OPENED + 1, KEY_LID_OPENED + 2));
    let skel = (skeleton::KEY_BRIGHTNESS_UP, skeleton::KEY_BRIGHTNESS_DOWN);
    assert_eq!((KEY_BRIGHTNESS_UP, KEY_BRIGHTNESS_DOWN), skel);
    assert!(is_shell_key(KEY_BRIGHTNESS_UP) && is_shell_key(KEY_BRIGHTNESS_DOWN));
}

#[test]
fn the_brightness_keys_go_to_the_shell_while_a_window_has_focus() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    world::set_focus(A);
    for code in [KEY_BRIGHTNESS_UP, KEY_BRIGHTNESS_UP, KEY_BRIGHTNESS_DOWN] {
        key_down(&mut ctx, code, 0);
        key_up(&mut ctx, code, 0);
    }
    let rx = sent();
    let keys =
        [(SHELL, KEY_BRIGHTNESS_UP), (SHELL, KEY_BRIGHTNESS_UP), (SHELL, KEY_BRIGHTNESS_DOWN)];
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), keys);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), keys);
    assert!(rx.iter().all(|r| r.pid != A), "the focused window saw a brightness key: {rx:?}");
}
