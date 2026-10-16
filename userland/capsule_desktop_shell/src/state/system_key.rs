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

//! What the shell does with a system key, the keys the input router hands it
//! whatever has focus (its route/shell_keys.rs). Every key the router sends
//! here is one of these, so none falls through to a rename or the Launchpad
//! search and types a character there.

use nonos_app_skeleton::{
    KEY_BRIGHTNESS_DOWN, KEY_BRIGHTNESS_UP, KEY_LID_CLOSED, KEY_LID_OPENED, KEY_POWER,
};

use super::volume::{volume_key, VolumeKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemKey {
    /// Mute, Volume Down or Volume Up: the master volume (state/volume.rs).
    Volume(VolumeKey),
    /// The power key, from a keyboard or the machine's ACPI power button:
    /// it asks to power off (state/power_off.rs).
    Power,
    /// The lid closed (true) or opened, from the kernel (state/lid.rs).
    Lid(bool),
    /// Brightness Up (true) or Down: the focused output's panel
    /// (state/brightness.rs).
    Brightness(bool),
}

/// The system key `code` names, or None for any other key.
pub fn system_key(code: u32) -> Option<SystemKey> {
    match code {
        KEY_POWER => Some(SystemKey::Power),
        KEY_LID_CLOSED | KEY_LID_OPENED => Some(SystemKey::Lid(code == KEY_LID_CLOSED)),
        KEY_BRIGHTNESS_UP | KEY_BRIGHTNESS_DOWN => {
            Some(SystemKey::Brightness(code == KEY_BRIGHTNESS_UP))
        }
        _ => volume_key(code).map(SystemKey::Volume),
    }
}

/// What the shell says when no service answers to `power.off`, so a power off
/// confirmed from the menu or this key has nowhere to go
/// (state/power_off.rs). Said rather than nothing, so a person who asked is
/// not left wondering whether it was heard.
pub const POWER_OFF_UNAVAILABLE: &[u8] = b"Power off is not available from the desktop";
