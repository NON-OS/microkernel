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

//! The layout chord consumed, act-once keys not repeated, and a code with
//! no key ignored.

use super::keyboard::keys;
use crate::keyboard::{cycles, Keyboard};

#[test]
fn ctrl_alt_space_cycles_the_layout_and_posts_nothing_of_its_own() {
    let mut kb = Keyboard::new();
    kb.key(29, 1); // left ctrl
    kb.key(56, 1); // left alt
    let before = keys().len();
    kb.key(57, 1);
    kb.key(57, 2);
    kb.key(57, 0);
    assert_eq!(keys().len(), 0, "{before} modifier posts already taken");
    assert_eq!(cycles(), 1);
}

#[test]
fn mute_and_power_act_once_per_press() {
    let mut kb = Keyboard::new();
    kb.key(113, 1);
    kb.key(113, 2);
    kb.key(113, 2);
    kb.key(113, 0);
    assert_eq!(keys().len(), 2);
    kb.key(0x2FF, 1);
    assert!(keys().is_empty(), "a code with no key posts nothing");
}
