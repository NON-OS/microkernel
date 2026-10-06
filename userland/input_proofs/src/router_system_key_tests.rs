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

//! The system keys (Mute, Volume Down, Volume Up, Power) go to the desktop
//! shell whatever has focus, press and release, as the reserved chord does: a
//! full screen game or a hung app must not swallow the volume.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::ps2_keycodes::{KEYCODE_MUTE, KEYCODE_POWER, KEYCODE_VOLUME_DOWN, KEYCODE_VOLUME_UP};
use crate::route::shell_keys::{
    is_shell_key, KEY_MUTE, KEY_POWER, KEY_VOLUME_DOWN, KEY_VOLUME_UP,
};
use crate::router_desk::*;
use crate::usb_usage_keycode::usage_keycode;

const A: u32 = 20;
const B: u32 = 21;
const SYSTEM: [u32; 4] = [KEY_MUTE, KEY_VOLUME_DOWN, KEY_VOLUME_UP, KEY_POWER];

#[test]
fn the_router_names_the_codes_the_drivers_post() {
    assert_eq!(SYSTEM, [KEYCODE_MUTE, KEYCODE_VOLUME_DOWN, KEYCODE_VOLUME_UP, KEYCODE_POWER]);
    assert_eq!(usage_keycode(0x7f), Some(KEY_MUTE));
    assert_eq!(usage_keycode(0x81), Some(KEY_VOLUME_DOWN));
    assert_eq!(usage_keycode(0x80), Some(KEY_VOLUME_UP));
    assert_eq!(usage_keycode(0x66), Some(KEY_POWER));
}

#[test]
fn only_the_system_keys_are_the_shells() {
    for code in SYSTEM {
        assert!(is_shell_key(code), "{code:#x}");
    }
    for code in [0x1B, u32::from(b'x'), 0x1101, 0x1201, 0x120A, 0x1300, 0x1305, 0x2000 | 0x7f] {
        assert!(!is_shell_key(code), "{code:#x} is the focused window's");
    }
}

#[test]
fn each_system_key_goes_to_the_shell_while_a_window_has_focus() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    world::set_focus(A);
    for code in SYSTEM {
        key_down(&mut ctx, code, 0);
        key_up(&mut ctx, code, 0);
    }
    let rx = sent();
    let to_shell: Vec<_> = SYSTEM.iter().map(|&c| (SHELL, c)).collect();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), to_shell);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), to_shell);
    assert!(rx.iter().all(|r| r.pid != A), "the focused window saw a system key: {rx:?}");
}

#[test]
fn a_held_volume_key_repeats_to_the_shell_and_a_typed_key_still_goes_to_focus() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    world::set_focus(A);
    key_down(&mut ctx, KEY_VOLUME_UP, 0);
    key_down(&mut ctx, KEY_VOLUME_UP, 0);
    key_down(&mut ctx, u32::from(b'x'), 0);
    key_down(&mut ctx, KEY_VOLUME_UP, 0);
    key_up(&mut ctx, KEY_VOLUME_UP, 0);
    key_up(&mut ctx, u32::from(b'x'), 0);
    let rx = sent();
    assert_eq!(
        of_kind(&rx, INPUT_KIND_KEY_DOWN),
        [
            (SHELL, KEY_VOLUME_UP),
            (SHELL, KEY_VOLUME_UP),
            (A, u32::from(b'x')),
            (SHELL, KEY_VOLUME_UP)
        ]
    );
    assert_eq!(
        of_kind(&rx, INPUT_KIND_KEY_UP),
        [(SHELL, KEY_VOLUME_UP), (A, u32::from(b'x'))],
        "no release went between the repeats"
    );
}

#[test]
fn a_volume_key_held_across_a_focus_change_stays_with_the_shell() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    window(&mut ctx, B, (400, 0, 300, 300));
    world::set_focus(A);
    key_down(&mut ctx, KEY_VOLUME_DOWN, 0);
    world::set_focus(B);
    key_down(&mut ctx, KEY_VOLUME_DOWN, 0);
    key_up(&mut ctx, KEY_VOLUME_DOWN, 0);
    let rx = sent();
    assert!(rx.iter().all(|r| r.pid == SHELL), "{rx:?}");
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN).len(), 2);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(SHELL, KEY_VOLUME_DOWN)]);
}

#[test]
fn a_system_key_release_with_no_press_on_record_goes_to_the_shell() {
    // The press went before the router was up, or before the shell was.
    // Any other key's orphan release goes to the window last seen focused.
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    world::set_focus(A);
    key_down(&mut ctx, u32::from(b'y'), 0);
    key_up(&mut ctx, u32::from(b'y'), 0);
    key_up(&mut ctx, KEY_MUTE, 0);
    key_up(&mut ctx, u32::from(b'x'), 0);
    let rx = sent();
    assert_eq!(
        of_kind(&rx, INPUT_KIND_KEY_UP),
        [(A, u32::from(b'y')), (SHELL, KEY_MUTE), (A, u32::from(b'x'))]
    );
}

#[test]
fn a_modifier_held_does_not_hand_a_system_key_to_the_window() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    world::set_focus(A);
    key_down(&mut ctx, KEY_POWER, MOD_CTRL | MOD_ALT);
    key_up(&mut ctx, KEY_POWER, MOD_CTRL | MOD_ALT);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(SHELL, KEY_POWER)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(SHELL, KEY_POWER)]);
}
