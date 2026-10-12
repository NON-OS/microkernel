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

//! Which part of the layout an input field can be, by the application
//! collection it is in (HID Usage Tables). A field in any other collection,
//! a gamepad's, a UPS's or a vendor's, is no part of it.

use super::input::Input;
use super::layout::Layout;
use super::{keys, media, pointer_take};

pub fn hand(app: u32, layout: &mut Layout, inp: &Input) {
    match app {
        // Generic desktop Keyboard and Keypad.
        0x0001_0006 | 0x0001_0007 => keys::take(&mut layout.keys, inp),
        // Consumer Control, then generic desktop System Control.
        0x000C_0001 => media::take(&mut layout.media[0], inp),
        0x0001_0080 => media::take(&mut layout.media[1], inp),
        // Generic desktop Pointer and Mouse; digitizer Digitizer, Pen and
        // Touch Screen.
        0x0001_0001 | 0x0001_0002 | 0x000D_0001 | 0x000D_0002 | 0x000D_0004 => {
            pointer_take::take(&mut layout.pointer, inp)
        }
        _ => {}
    }
}
