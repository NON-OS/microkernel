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

pub const KEY_BACKSPACE: u32 = 0x08;
pub const KEY_TAB: u32 = 0x09;
pub const KEY_ENTER: u32 = 0x0D;
pub const KEY_ESC: u32 = 0x1B;

/* Navigation and function codes mirror the PS/2 driver's published keycode
 * table (capsule_driver_ps2_input keymap/set1/keycodes.rs), hand-synced. */
pub const KEY_F6: u32 = 0x1106;
pub const KEY_UP: u32 = 0x1201;
pub const KEY_DOWN: u32 = 0x1202;
pub const KEY_LEFT: u32 = 0x1203;
pub const KEY_RIGHT: u32 = 0x1204;
pub const KEY_HOME: u32 = 0x1205;
pub const KEY_END: u32 = 0x1206;
pub const KEY_PAGE_UP: u32 = 0x1207;
pub const KEY_PAGE_DOWN: u32 = 0x1208;
pub const KEY_INSERT: u32 = 0x1209;
pub const KEY_DELETE: u32 = 0x120A;
/// F1; F2 to F12 follow in order.
pub const KEY_F1: u32 = 0x1101;
pub const KEY_F12: u32 = 0x110C;
/* The system keys: the laptop's Fn volume keys and the power button. The
 * input router hands them to the desktop shell whatever has focus, so an app
 * never sees them; named here so every holder of the table agrees. */
pub const KEY_MUTE: u32 = 0x1301;
pub const KEY_VOLUME_DOWN: u32 = 0x1302;
pub const KEY_VOLUME_UP: u32 = 0x1303;
pub const KEY_POWER: u32 = 0x1304;
