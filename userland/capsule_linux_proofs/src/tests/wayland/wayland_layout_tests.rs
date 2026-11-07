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

//! A sign typed on any layout reaches the client as that sign. The driver
//! sends the character its layout gave and the modifiers held; the client
//! reads the key through the US keymap, so the key goes with the shift
//! that US key needs, not the one the person held.

use crate::evdev::{evdev_of, us_shift};
use crate::xkb_mods::{as_us, key_say, xkb_masks, ALT, ALTGR, CTRL, SHIFT};

/// The US keymap's number row and signs, written out apart from the
/// bridge's table: evdev code, plain, shifted.
const US: [(u32, u8, u8); 22] = [
    (2, b'1', b'!'),
    (3, b'2', b'@'),
    (4, b'3', b'#'),
    (5, b'4', b'$'),
    (6, b'5', b'%'),
    (7, b'6', b'^'),
    (8, b'7', b'&'),
    (9, b'8', b'*'),
    (10, b'9', b'('),
    (11, b'0', b')'),
    (12, b'-', b'_'),
    (13, b'=', b'+'),
    (26, b'[', b'{'),
    (27, b']', b'}'),
    (39, b';', b':'),
    (40, b'\'', b'"'),
    (41, b'`', b'~'),
    (43, b'\\', b'|'),
    (51, b',', b'<'),
    (52, b'.', b'>'),
    (53, b'/', b'?'),
    (57, b' ', b' '),
];

/// What a client reading the US keymap types for `key` under `depressed`.
fn client_types(key: u32, depressed: u32) -> Option<u8> {
    let (_, plain, shifted) = US.iter().find(|k| k.0 == key)?;
    Some(if depressed & 1 != 0 { *shifted } else { *plain })
}

/// The sign `c`, typed holding `flags`, as the client reads it.
fn typed(c: u8, flags: u16) -> Option<u8> {
    let told = xkb_masks(flags);
    let masks = as_us(us_shift(c.into()), told);
    client_types(evdev_of(c.into())?, masks.0)
}

#[test]
fn german_signs_are_typed_as_themselves() {
    // QWERTZ: '+' and '#' plain, '/' and '*' shifted, '@' '{' '\' with AltGr.
    let de = [
        (b'+', 0),
        (b'#', 0),
        (b'-', 0),
        (b'/', SHIFT),
        (b'*', SHIFT),
        (b'=', SHIFT),
        (b'?', SHIFT),
        (b'\'', SHIFT),
        (b'@', ALTGR),
        (b'{', ALTGR),
        (b'[', ALTGR),
        (b'\\', ALTGR),
        (b'|', ALTGR),
        (b'~', ALTGR),
    ];
    for (c, flags) in de {
        assert_eq!(typed(c, flags), Some(c), "{} under {flags:#x}", c as char);
    }
}

#[test]
fn french_number_row_is_typed_as_itself() {
    // AZERTY: signs plain on the number row, digits shifted.
    for (c, flags) in [(b'&', 0), (b'"', 0), (b'(', 0), (b'-', 0), (b'_', 0), (b')', 0)] {
        assert_eq!(typed(c, flags), Some(c), "{}", c as char);
    }
    for c in b'0'..=b'9' {
        assert_eq!(typed(c, SHIFT), Some(c), "{}", c as char);
    }
}

#[test]
fn the_us_layout_is_said_as_it_was() {
    for &(_, plain, shifted) in &US {
        assert_eq!(as_us(us_shift(plain.into()), xkb_masks(0)), xkb_masks(0));
        if plain != shifted {
            assert_eq!(as_us(us_shift(shifted.into()), xkb_masks(SHIFT)), xkb_masks(SHIFT));
        }
    }
}

#[test]
fn letters_and_named_keys_keep_the_held_masks() {
    for code in [u32::from(b'a'), u32::from(b'Q'), 0x1201, 0x1001, 0x0D, 0x1B, 0x20AC] {
        assert_eq!(us_shift(code), None, "{code:#x}");
        let told = xkb_masks(SHIFT | ALTGR | CTRL);
        assert_eq!(as_us(us_shift(code), told), told);
    }
}

#[test]
fn ctrl_and_alt_stay_with_a_sign() {
    // Ctrl and '+' on QWERTZ (zoom in): Ctrl, shift and '='.
    let (depressed, _) = as_us(us_shift(b'+'.into()), xkb_masks(CTRL));
    assert_eq!(depressed, 1 | 1 << 2);
    let (depressed, _) = as_us(us_shift(b'-'.into()), xkb_masks(CTRL | ALT | SHIFT));
    assert_eq!(depressed, 1 << 2 | 1 << 3, "shift dropped, Ctrl and Alt kept");
}

#[test]
fn the_held_masks_are_told_again_after_a_shifted_sign() {
    // '/' with Shift held on QWERTZ is said unshifted; Shift is held again
    // after it, for a Shift+click that follows.
    let told = xkb_masks(SHIFT);
    let typed = as_us(us_shift(b'/'.into()), told);
    let say = key_say(53, true, true, (told, typed));
    assert_eq!((say.before, say.key, say.after), (Some((0, 0)), true, Some(told)));
    let same = key_say(53, true, true, (xkb_masks(0), xkb_masks(0)));
    assert_eq!(same.after, None, "nothing again when nothing changed");
}

#[test]
fn a_repeat_tells_the_held_masks_again_after_it() {
    let key = include_str!("../../../../capsule_linux/src/linux/wayland/input/key.rs");
    let at = key.find("fn repeat(").expect("repeat");
    let body = &key[at..at + key[at..].find("\n}\n").expect("end")];
    assert!(body.contains("mods(guest, keyboards, as_us(shift, told));"));
    assert!(body.trim_end().ends_with("mods(guest, keyboards, told);"));
}
