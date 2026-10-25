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

//! A Bluetooth keyboard's source and bond bits (15 and 14), the flags the
//! input router reads to keep an unauthenticated keyboard off the secure
//! path, ride with every modifier and Caps Lock state, the modifiers whole.

use nonos_keymap::HeldKeys;

use crate::hid::key_event::KeyEvent;
use crate::hid::key_post::key_posts;
use crate::key_post_seq::flags_before;

#[test]
fn a_bluetooth_keyboards_source_and_bond_bits_ride_with_every_modifier() {
    let bt = 1 << 15 | 1 << 14;
    for modifiers in 0..=255u8 {
        for caps in [false, true] {
            let ev = KeyEvent { scancode: 0x04, ascii: 0, modifiers, pressed: true, caps };
            let [_, Some(p)] = key_posts(ev, &mut HeldKeys::new(), 0x61, bt) else { panic!() };
            assert_eq!(p.flags, flags_before(modifiers, caps) | bt, "{modifiers:#04x} {caps}");
        }
    }
}
