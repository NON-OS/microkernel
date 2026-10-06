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

//! The laptop's Fn volume keys and the power key, from the bytes an i8042
//! hands the driver to the codes it posts.
//!
//! An HP 15s (Gemini Lake) sends its Fn volume keys as E0-prefixed scan set 1
//! codes, the ones a Windows multimedia keyboard sends: E0 20 Mute, E0 2E
//! Volume Down, E0 30 Volume Up, and E0 5E for ACPI Power. The same byte
//! without the prefix is a letter (20 is D, 2E is C, 30 is B), so the prefix
//! is what these pin as much as the codes.

use crate::keymap::once::acts_once;
use crate::keymap::set1::{
    KEYCODE_F1, KEYCODE_MUTE, KEYCODE_POWER, KEYCODE_VOLUME_DOWN, KEYCODE_VOLUME_UP,
};
use crate::keymap::translate::translate;
use crate::ring::{FLAG_BREAK, FLAG_E0_PREFIX};

const SYSTEM_KEYS: [(u8, u32); 4] = [
    (0x20, KEYCODE_MUTE),
    (0x2E, KEYCODE_VOLUME_DOWN),
    (0x30, KEYCODE_VOLUME_UP),
    (0x5E, KEYCODE_POWER),
];

/// What the driver posts for one byte after a prefix, as absorb builds the
/// flags: the break bit from the byte, the prefix from the byte before.
fn decode(byte: u8, e0: bool) -> Option<(u32, bool)> {
    let mut flags = 0;
    if byte & 0x80 != 0 {
        flags |= FLAG_BREAK;
    }
    if e0 {
        flags |= FLAG_E0_PREFIX;
    }
    translate(byte, flags).map(|t| (t.keycode, t.is_release))
}

#[test]
fn the_e0_make_codes_press_the_system_keys() {
    for (scan, code) in SYSTEM_KEYS {
        assert_eq!(decode(scan, true), Some((code, false)), "E0 {scan:02X}");
    }
}

#[test]
fn the_e0_break_codes_release_them() {
    for (scan, code) in SYSTEM_KEYS {
        assert_eq!(decode(scan | 0x80, true), Some((code, true)), "E0 {:02X}", scan | 0x80);
    }
}

#[test]
fn without_the_prefix_the_same_bytes_stay_letters() {
    assert_eq!(decode(0x20, false), Some((u32::from(b'd'), false)));
    assert_eq!(decode(0x2E, false), Some((u32::from(b'c'), false)));
    assert_eq!(decode(0x30, false), Some((u32::from(b'b'), false)));
    assert_eq!(decode(0x5E, false), None, "no base key sits at 5E");
}

#[test]
fn the_system_keys_sit_in_a_block_of_their_own() {
    // 0x1001.. modifiers, 0x1101.. F keys, 0x1201.. navigation, 0x1301.. these.
    let codes = [KEYCODE_MUTE, KEYCODE_VOLUME_DOWN, KEYCODE_VOLUME_UP, KEYCODE_POWER];
    assert_eq!(codes, [0x1301, 0x1302, 0x1303, 0x1304]);
    for code in codes {
        assert!(code > KEYCODE_F1 && !(0x20..=0x7E).contains(&code));
    }
    // No other E0 or base code posts one of them.
    for byte in 0u8..=0x7F {
        for e0 in [false, true] {
            if let Some((code, _)) = decode(byte, e0) {
                let ours = SYSTEM_KEYS.iter().any(|&(s, _)| e0 && s == byte);
                assert_eq!(codes.contains(&code), ours, "{} {byte:02X}", if e0 { "E0" } else { "" });
            }
        }
    }
}

#[test]
fn mute_and_power_act_once_and_the_volume_steps_repeat() {
    // A held key repeats as more make codes; the driver drops these two's.
    assert!(acts_once(KEYCODE_MUTE));
    assert!(acts_once(KEYCODE_POWER));
    assert!(!acts_once(KEYCODE_VOLUME_UP), "held, Volume Up keeps stepping");
    assert!(!acts_once(KEYCODE_VOLUME_DOWN));
    assert!(!acts_once(u32::from(b'a')));
}
