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

//! A client is told a key is down once, and up once: a driver's hardware
//! repeats are not sent as presses, and the client repeats a held key itself.

use crate::held_keys::{HeldKeys, MOST_HELD};

#[test]
fn a_repeat_is_not_news_and_a_release_is_once() {
    let mut held = HeldKeys::new();
    assert!(held.press(30));
    assert!(!held.press(30) && !held.press(30), "repeats");
    assert!(held.press(42), "another key goes down");
    assert!(held.release(30));
    assert!(!held.release(30), "a second release");
    assert!(held.press(30), "pressed again after its release");
}

#[test]
fn a_release_of_a_key_never_down_is_not_sent() {
    let mut held = HeldKeys::new();
    assert!(!held.release(28));
    assert!(held.press(28) && held.release(28));
}

#[test]
fn no_more_than_most_held_are_down() {
    let mut held = HeldKeys::new();
    for key in 0..MOST_HELD as u32 {
        assert!(held.press(key), "{key}");
    }
    assert!(!held.press(500), "past the limit");
    assert!(held.release(3) && held.press(500));
}
