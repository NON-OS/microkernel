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

//! The system keys: Mute, Volume Down, Volume Up and Power. They act on the
//! machine, not on a window, so they go to the desktop shell whatever has
//! focus, as the reserved chord does (chord.rs): a full screen game or a hung
//! app must not swallow the volume, and no app has a use for them as input.
//! Their releases follow them there through the press table.

/// The codes the keyboard drivers post for them (capsule_driver_ps2_input
/// keymap/set1/keycodes.rs, capsule_driver_usb_hid hid/usage_keycode.rs) and
/// app_skeleton names KEY_MUTE to KEY_POWER.
pub const KEY_MUTE: u32 = 0x1301;
pub const KEY_VOLUME_DOWN: u32 = 0x1302;
pub const KEY_VOLUME_UP: u32 = 0x1303;
pub const KEY_POWER: u32 = 0x1304;

/// Whether `code` is a key the shell takes whatever has focus.
pub fn is_shell_key(code: u32) -> bool {
    matches!(code, KEY_MUTE | KEY_VOLUME_DOWN | KEY_VOLUME_UP | KEY_POWER)
}
