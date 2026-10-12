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

//! A press of the Power key, posted to the desktop as the fixed power
//! button's is, for a power button that reports through AML (PNP0C0C) and
//! for a thermal zone at its critical trip point.

use crate::kernel_core::surface_registry::{try_post_input, InputEvent};

const KEYCODE_POWER: u32 = 0x1304;
const KEY_DOWN: u16 = 0;
const KEY_UP: u16 = 1;
const TRIES: usize = 8;

fn post(kind: u16) -> bool {
    let ev = InputEvent {
        kind,
        flags: 0,
        code: KEYCODE_POWER,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    };
    (0..TRIES).any(|_| matches!(try_post_input(ev), Some(Ok(()))))
}

/// False when the input ring stayed busy or full; the capsule hears so and
/// may ask again.
pub fn press() -> bool {
    post(KEY_DOWN) && {
        // The shell acts on the press; a lost release only costs whoever
        // tracks held keys.
        let _ = post(KEY_UP);
        true
    }
}
