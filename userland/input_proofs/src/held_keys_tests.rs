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

//! The keyboard drivers post a key's release with the code its press went
//! down with (nonos_keymap::HeldKeys), whatever the modifiers and layout
//! resolve it to by the time it comes up.

use nonos_keymap::{resolve, HeldKeys, KeyPosts, Layout};

const A_KEY: u32 = b'a' as u32;
const ENTER_USAGE: u32 = 0x28;

fn posts(release_first: Option<u32>, code: Option<u32>) -> KeyPosts {
    KeyPosts { release_first, code }
}

#[test]
fn a_letter_let_go_after_shift_comes_up_as_the_letter_it_went_down_as() {
    let mut held = HeldKeys::new();
    let down = resolve(A_KEY, true, false, false, Layout::Us);
    assert_eq!(held.event(A_KEY, false, down), posts(None, Some(u32::from(b'A'))));
    // Resolved again at its release, with Shift already up, the key is 'a':
    // the code the drivers used to post, which pairs with nothing.
    let up = resolve(A_KEY, false, false, false, Layout::Us);
    assert_eq!(up, u32::from(b'a'));
    assert_eq!(held.event(A_KEY, true, up), posts(None, Some(u32::from(b'A'))));
}

#[test]
fn enter_comes_up_with_the_code_it_went_down_with() {
    // The USB driver resolves a release with no ASCII byte, so Enter went
    // down as 0x0D and came up as its raw usage 0x2028.
    let mut held = HeldKeys::new();
    assert_eq!(held.event(ENTER_USAGE, false, 0x0D), posts(None, Some(0x0D)));
    assert_eq!(held.event(ENTER_USAGE, true, 0x2000 | ENTER_USAGE), posts(None, Some(0x0D)));
}

#[test]
fn a_repeat_that_resolves_to_another_code_releases_the_old_one_first() {
    let mut held = HeldKeys::new();
    assert_eq!(held.event(A_KEY, false, u32::from(b'a')), posts(None, Some(u32::from(b'a'))));
    assert_eq!(held.event(A_KEY, false, u32::from(b'a')), posts(None, Some(u32::from(b'a'))));
    // Shift goes down while the key repeats.
    assert_eq!(
        held.event(A_KEY, false, u32::from(b'A')),
        posts(Some(u32::from(b'a')), Some(u32::from(b'A')))
    );
    assert_eq!(held.event(A_KEY, true, u32::from(b'a')), posts(None, Some(u32::from(b'A'))));
}

#[test]
fn a_consumed_press_posts_no_release() {
    let mut held = HeldKeys::new();
    let space = u32::from(b' ');
    assert_eq!(held.press(space, 0), None);
    assert_eq!(held.event(space, true, space), posts(None, None));
    // Not held any more: a fresh press posts again.
    assert_eq!(held.event(space, false, space), posts(None, Some(space)));
}

#[test]
fn a_key_the_layout_leaves_empty_posts_nothing() {
    let mut held = HeldKeys::new();
    assert_eq!(held.event(nonos_keymap::KEY_ISO, false, 0), posts(None, None));
    assert_eq!(held.event(nonos_keymap::KEY_ISO, true, 0), posts(None, None));
}

#[test]
fn past_a_full_table_a_release_resolves_on_its_own() {
    let mut held = HeldKeys::new();
    for key in 1..=16 {
        held.event(key, false, key + 0x100);
    }
    assert_eq!(held.event(99, false, 0x41), posts(None, Some(0x41)));
    assert_eq!(held.event(99, true, 0x61), posts(None, Some(0x61)));
    // The keys that fit keep their codes.
    assert_eq!(held.event(16, true, 0), posts(None, Some(16 + 0x100)));
}

#[test]
fn a_repeat_is_told_from_a_fresh_press() {
    // The PS/2 driver toggles Caps Lock and cycles the layout only on a
    // fresh press; the keyboard's own repeats of the key are held already.
    const CAPS: u32 = 0x1009;
    let mut held = HeldKeys::new();
    assert_eq!(held.code(CAPS), None);
    held.event(CAPS, false, CAPS);
    assert_eq!(held.code(CAPS), Some(CAPS));
    held.event(CAPS, true, CAPS);
    assert_eq!(held.code(CAPS), None);
}

#[test]
fn the_other_shift_still_held_keeps_shift_down() {
    const LSHIFT: u32 = 0x1003;
    const RSHIFT: u32 = 0x1004;
    let shift = |k: u32| k == LSHIFT || k == RSHIFT;
    let mut held = HeldKeys::new();
    held.event(LSHIFT, false, LSHIFT);
    held.event(RSHIFT, false, RSHIFT);
    // Left Shift comes up: Right Shift is still down.
    assert!(held.other_held(LSHIFT, shift));
    held.event(LSHIFT, true, LSHIFT);
    // Right Shift comes up: nothing else holds Shift.
    assert!(!held.other_held(RSHIFT, shift));
}
