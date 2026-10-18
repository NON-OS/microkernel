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

//! A device that goes away lets go of everything it held, at once: every key
//! comes up with the code it went down with and its repeat ends, and every
//! button comes up, so a drag ends. The last key held used to repeat into
//! the focused window for up to 30 s and never come up.

use super::drop_port::drop_port;
use crate::descriptors::HidKind;
use crate::endpoints::{ep, posted};
use crate::hid::keyboard::BootReport;
use crate::hid::Keyboard;
use nonos_libc::{reset, INPUT_KIND_KEY_UP as KEY_UP};

const A: u32 = b'A' as u32;
const B: u32 = b'B' as u32;

#[test]
fn unplugging_a_keyboard_with_keys_held_releases_each_with_its_code() {
    reset();
    let mut eps = vec![ep(3, 1, 3, HidKind::Keyboard), ep(4, 2, 3, HidKind::Keyboard)];
    // Shift with A and B on the keyboard that goes; C on the one that stays.
    eps[0].dev.feed(&[0x02, 0, 0x04, 0x05, 0, 0, 0, 0]);
    eps[1].dev.feed(&[0, 0, 0x06, 0, 0, 0, 0, 0]);
    let _ = posted();
    assert_eq!(drop_port(&mut eps, 3), vec![1]);
    assert_eq!(posted(), vec![(KEY_UP, A), (KEY_UP, B)]);
    assert_eq!(eps.len(), 1, "the other keyboard's C stays down");
}

#[test]
fn a_released_keyboard_repeats_nothing_after() {
    reset();
    let mut kbd = Keyboard::new();
    kbd.apply(BootReport { modifiers: 0, keys: [0x04, 0, 0, 0, 0, 0] });
    kbd.tick(0);
    kbd.tick(600);
    assert_eq!(posted().len(), 2, "the press and one repeat");
    kbd.release_all();
    assert_eq!(posted(), vec![(KEY_UP, u32::from(b'a'))]);
    for t in (600..31_000).step_by(11) {
        kbd.tick(t);
    }
    assert_eq!(posted(), vec![], "nothing repeats once the device is gone");
}
