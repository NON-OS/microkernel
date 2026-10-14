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

//! Which key toggles which of the keyboard's lock LEDs.

use crate::keymap::locks::{lock_bit, LED_CAPS, LED_NUM, LED_SCROLL};
use crate::keymap::set1::{KEYCODE_CAPS, KEYCODE_LSHIFT, KEYCODE_NUMLK, KEYCODE_SCROLL};

#[test]
fn each_lock_key_owns_its_led_bit_and_no_other_key_has_one() {
    assert_eq!(lock_bit(KEYCODE_CAPS), Some(LED_CAPS));
    assert_eq!(lock_bit(KEYCODE_NUMLK), Some(LED_NUM));
    assert_eq!(lock_bit(KEYCODE_SCROLL), Some(LED_SCROLL));
    assert_eq!((LED_SCROLL, LED_NUM, LED_CAPS), (1, 2, 4), "the 0xED bitmap");
    assert_eq!(lock_bit(KEYCODE_LSHIFT), None);
    assert_eq!(lock_bit(u32::from(b'a')), None);
}
