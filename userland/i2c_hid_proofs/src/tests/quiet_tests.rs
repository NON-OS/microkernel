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

//! The input path posts input and nothing else (plan/50 C5, C8): no
//! diagnostic event beside each report, no frame on the console.

use nonos_i2cmodel::touchpad::Touch;
use nonos_libc::{take_debug, take_events};

use super::driven::{bound, frame};

#[test]
fn frames_post_input_events_only_and_print_nothing() {
    let (r, mut s) = bound();
    take_debug();
    for x in [100, 140, 180, 220, 260, 300, 340, 380] {
        frame(&r, &mut s, Touch::finger(x, 100));
    }
    frame(&r, &mut s, Touch::lifted());
    let events = take_events();
    assert!(!events.is_empty(), "nothing posted for a finger that moved");
    assert!(events.iter().all(|e| e.kind < 0x100), "a diagnostic was posted: {events:?}");
    let lines = take_debug();
    assert!(lines.is_empty(), "frames went to the console: {lines:?}");
}
