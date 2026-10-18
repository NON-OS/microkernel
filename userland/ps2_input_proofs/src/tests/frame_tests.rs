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

//! Framing each keyboard byte by the prefixes before it, then decoding it,
//! as absorb does: Pause is one key and never Ctrl+Num Lock, Print Screen
//! and Break are keys of their own, the fake shifts post nothing, and the
//! keyboard's ACK and RESEND are answers, not keys.

use crate::keymap::set1::{KEYCODE_LCTRL, KEYCODE_NUMLK, KEYCODE_PAUSE, KEYCODE_PRINT};
use crate::keymap::translate::translate;
use crate::prefix::{Prefix, Scan};

/// What a byte stream posts, as (keycode, released), and the replies seen.
fn decode(bytes: &[u8]) -> (Vec<(u32, bool)>, Vec<u8>) {
    let (mut keys, mut replies, mut p) = (Vec::new(), Vec::new(), Prefix::IDLE);
    for &b in bytes {
        match p.take(b) {
            Scan::Key { scancode, flags } => {
                keys.extend(translate(scancode, flags).map(|t| (t.keycode, t.is_release)))
            }
            Scan::Reply(r) => replies.push(r),
            Scan::Pending => {}
        }
    }
    (keys, replies)
}

#[test]
fn pause_is_one_press_and_one_release_of_pause() {
    let (keys, _) = decode(&[0xE1, 0x1D, 0x45, 0xE1, 0x9D, 0xC5]);
    assert_eq!(keys, [(KEYCODE_PAUSE, false), (KEYCODE_PAUSE, true)]);
    assert!(!keys.iter().any(|k| k.0 == KEYCODE_LCTRL || k.0 == KEYCODE_NUMLK));
}

#[test]
fn print_screen_and_break_are_keys_and_the_fake_shifts_post_nothing() {
    let (keys, _) = decode(&[0xE0, 0x2A, 0xE0, 0x37, 0xE0, 0xB7, 0xE0, 0xAA]);
    assert_eq!(keys, [(KEYCODE_PRINT, false), (KEYCODE_PRINT, true)]);
    let (keys, _) = decode(&[0xE0, 0x46, 0xE0, 0xC6]);
    assert_eq!(keys, [(KEYCODE_PAUSE, false), (KEYCODE_PAUSE, true)], "Ctrl+Break");
    assert_eq!(decode(&[0x54, 0xD4]).0, [(KEYCODE_PRINT, false), (KEYCODE_PRINT, true)]);
}

#[test]
fn the_keyboards_answers_are_not_keys() {
    let (keys, replies) = decode(&[0xFA, 0x1E, 0xFE, 0x9E]);
    assert_eq!(replies, [0xFA, 0xFE]);
    assert_eq!(keys, [(u32::from(b'a'), false), (u32::from(b'a'), true)]);
}

#[test]
fn an_answer_inside_pause_is_an_answer_and_pause_stays_whole() {
    // A key's bytes and a command's answer share the output buffer, so an
    // ACK can land between E1 1D and 45; the 45 must still end Pause.
    let (keys, replies) = decode(&[0xE1, 0x1D, 0xFA, 0x45, 0xE1, 0x9D, 0xFE, 0xC5]);
    assert_eq!(replies, [0xFA, 0xFE]);
    assert_eq!(keys, [(KEYCODE_PAUSE, false), (KEYCODE_PAUSE, true)], "Num Lock, not Pause");
}
