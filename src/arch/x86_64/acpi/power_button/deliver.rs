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

//! A press into the input ring, as the Power key.

use crate::kernel_core::surface_registry::{try_post_input, InputEvent};

/// The Power key's code (KEYCODE_POWER in the keyboard drivers and
/// app_skeleton), and the key event kinds (nonos_libc INPUT_KIND_KEY_*).
const KEYCODE_POWER: u32 = 0x1304;
const KEY_DOWN: u16 = 0;
const KEY_UP: u16 = 1;

/// The press and its release into the input ring. False when the ring was
/// busy or full, to try again next tick.
pub(super) fn deliver() -> bool {
    let key = |kind| InputEvent {
        kind,
        flags: 0,
        code: KEYCODE_POWER,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    };
    if !matches!(try_post_input(key(KEY_DOWN)), Some(Ok(()))) {
        return false;
    }
    // The shell acts on the press. The release is for whoever tracks held
    // keys, and a ring still busy after a few tries loses only that.
    for _ in 0..4 {
        if matches!(try_post_input(key(KEY_UP)), Some(Ok(()))) {
            break;
        }
    }
    true
}
