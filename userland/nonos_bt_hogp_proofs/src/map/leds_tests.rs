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

//! The lock LEDs in an output report, and events from what reports hold.

use super::maps::{global, local, main, KEYBOARD};
use nonos_bt_hogp::report::map::{leds, parse};
use nonos_bt_hogp::report::{Held, Input::*, CAPS_LOCK, NUM_LOCK, SCROLL_LOCK};

#[test]
fn each_lock_lights_its_led_usage() {
    let l = parse(&KEYBOARD).unwrap();
    assert_eq!(leds(&l, 1, 0), Some(vec![0]));
    assert_eq!(leds(&l, 1, CAPS_LOCK), Some(vec![0x02]));
    assert_eq!(leds(&l, 1, NUM_LOCK | SCROLL_LOCK), Some(vec![0x05]));
    assert_eq!(leds(&l, 2, CAPS_LOCK), None, "report 2 has no output");
}

#[test]
fn leds_go_where_their_usages_say() {
    // Padding first, then Caps Lock alone, then Num Lock.
    let pad = [global(7, 2), global(9, 1), main(9, 1)].concat();
    let led = |u: u8| [global(0, 8), local(0, u), global(7, 1), main(9, 2)].concat();
    let l = parse(&[pad, led(2), led(1), global(7, 4), main(9, 1)].concat()).unwrap();
    assert_eq!(leds(&l, 0, CAPS_LOCK), Some(vec![0b0000_0100]));
    assert_eq!(leds(&l, 0, NUM_LOCK | CAPS_LOCK), Some(vec![0b0000_1100]));
    let none = [global(0, 7), local(0, 4), global(7, 8), global(9, 1), main(9, 2)].concat();
    assert_eq!(leds(&parse(&none).unwrap(), 0, CAPS_LOCK), None, "no LED usage");
}

#[test]
fn held_keys_become_releases_then_presses() {
    let mut h = Held::default();
    assert_eq!(h.to(vec![0xe1, 0x04]), [KeyDown(0xe1), KeyDown(0x04)]);
    assert_eq!(h.to(vec![0xe1, 0x05]), [KeyUp(0x04), KeyDown(0x05)]);
    assert_eq!(h.to(vec![0x05]), [KeyUp(0xe1)]);
    assert_eq!(h.release_all(), [KeyUp(0x05)]);
    assert_eq!(h.release_all(), []);
}
