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
//! Whether the digitizer is a touchscreen or a touchpad: the usage of its
//! first Application collection on the Digitizers page, 04h Touch Screen
//! or 05h Touch Pad (HID Usage Tables, as recalled, unverified). Anything
//! else is taken as a pad, the path every pad took before.

use super::item::{next, TYPE_GLOBAL, TYPE_LOCAL, TYPE_MAIN};

const PAGE_DIGITIZER: u16 = 0x0D;
const TOUCH_SCREEN: u32 = 0x04;
const TOUCH_PAD: u32 = 0x05;
const COLLECTION: u8 = 0xA;
const APPLICATION: u32 = 0x01;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Surface {
    #[default]
    Pad,
    Screen,
}

pub fn surface(desc: &[u8]) -> Surface {
    let (mut page, mut usage, mut i) = (0u16, 0u32, 0);
    while let Some(item) = next(desc, &mut i) {
        match (item.btype, item.tag) {
            (TYPE_GLOBAL, 0x0) => page = item.data as u16,
            (TYPE_LOCAL, 0x0) => usage = item.data,
            (TYPE_MAIN, COLLECTION) if item.data == APPLICATION && page == PAGE_DIGITIZER => {
                match usage {
                    TOUCH_SCREEN => return Surface::Screen,
                    TOUCH_PAD => return Surface::Pad,
                    _ => {}
                }
            }
            (TYPE_MAIN, _) => usage = 0,
            _ => {}
        }
    }
    Surface::Pad
}
