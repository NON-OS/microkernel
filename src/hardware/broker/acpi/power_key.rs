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
//! for a thermal zone at its critical trip point; and the lid's two keys
//! (F-91), one press when it closes and another when it opens, for the
//! shell's lock; and the output's brightness keys (F-96), for the shell to
//! ask the display engine. The input router hands these keys to the shell.

use crate::kernel_core::surface_registry::{try_post_input, InputEvent};

const KEYCODE_POWER: u32 = 0x1304;
pub const KEY_LID_CLOSED: u32 = 0x1305;
pub const KEY_LID_OPENED: u32 = 0x1306;
pub const KEY_BRIGHTNESS_UP: u32 = 0x1307;
pub const KEY_BRIGHTNESS_DOWN: u32 = 0x1308;
const KEY_DOWN: u16 = 0;
const KEY_UP: u16 = 1;
const TRIES: usize = 8;

fn post(code: u32, kind: u16) -> bool {
    let ev =
        InputEvent { kind, flags: 0, code, x: 0, y: 0, delta_x: 0, delta_y: 0, timestamp_ns: 0 };
    (0..TRIES).any(|_| matches!(try_post_input(ev), Some(Ok(()))))
}

/// False when the input ring stayed busy or full; the capsule hears so and
/// may ask again.
pub fn press() -> bool {
    tap(KEYCODE_POWER)
}

/// The lid's key for `open`; false as `press` is.
pub fn lid(open: bool) -> bool {
    tap(if open { KEY_LID_OPENED } else { KEY_LID_CLOSED })
}

/// A brightness key from the output's Notify; false as `press` is.
pub fn brightness(up: bool) -> bool {
    tap(if up { KEY_BRIGHTNESS_UP } else { KEY_BRIGHTNESS_DOWN })
}

fn tap(code: u32) -> bool {
    post(code, KEY_DOWN) && {
        // The shell acts on the press; a lost release only costs whoever
        // tracks held keys.
        let _ = post(code, KEY_UP);
        true
    }
}
