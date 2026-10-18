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

//! The system keys' codes sit in a block no other key reaches, and the ones
//! a hold must not repeat act once.

use super::keymap_tests::{decode, SYSTEM_KEYS};
use crate::keymap::once::acts_once;
use crate::keymap::set1::{
    KEYCODE_F1, KEYCODE_MUTE, KEYCODE_POWER, KEYCODE_VOLUME_DOWN, KEYCODE_VOLUME_UP,
};

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
                assert_eq!(
                    codes.contains(&code),
                    ours,
                    "{} {byte:02X}",
                    if e0 { "E0" } else { "" }
                );
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
