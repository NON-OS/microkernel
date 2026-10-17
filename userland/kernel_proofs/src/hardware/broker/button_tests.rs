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

//! The fixed power button has one reader: the kernel's poll, until the
//! ACPI capsule that took it is the live one; a capsule that died, or a
//! restarted one that has not taken it again, gives it back to the poll.

use super::acpi::button_owner::{press_rule, rule, stands_down, take, Press};

#[test]
fn the_poll_stands_down_only_for_the_live_taker() {
    for taken in [0u32, 7, 9] {
        for alive in [0u32, 7, 9] {
            assert_eq!(rule(taken, alive), taken != 0 && taken == alive, "{taken} {alive}");
        }
    }
    assert!(!stands_down(7), "nobody took it yet");
    take(7);
    assert!(stands_down(7), "the live capsule took it");
    assert!(!stands_down(0), "it died: the poll reads again");
    assert!(!stands_down(9), "restarted, not yet taken again");
    take(9);
    assert!(stands_down(9));
}

/// F-107: before any capsule served the button a press is the desktop's
/// Power key; after, the power-off it would have run.
#[test]
fn a_press_before_any_capsule_is_the_desktops() {
    assert_eq!(press_rule(false), Press::Desktop);
    assert_eq!(press_rule(true), Press::PowerOff);
}

#[path = "s5_choice_tests.rs"]
mod s5_choice;
