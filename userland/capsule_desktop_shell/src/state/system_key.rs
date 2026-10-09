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

use nonos_app_skeleton::KEY_POWER;

use super::volume::{volume_key, VolumeKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemKey {
    /// Mute, Volume Down or Volume Up: the master volume (state/volume.rs).
    Volume(VolumeKey),
    /// The power key, from a keyboard or the machine's ACPI power button.
    Power,
}

/// The system key `code` names, or None for any other key.
pub fn system_key(code: u32) -> Option<SystemKey> {
    if code == KEY_POWER {
        return Some(SystemKey::Power);
    }
    volume_key(code).map(SystemKey::Volume)
}

/// What the power key says. The desktop has no way to power off yet:
/// capsule_power is built but not spawned, and the shell offers no Shut Down
/// (docs/handbook/desktop/0.9.2-notes.md). The key says so rather than doing
/// nothing, so a person pressing it is not left wondering whether it was
/// heard.
pub const POWER_OFF_UNAVAILABLE: &[u8] = b"Power off is not available from the desktop";
