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

use nonos_policy_proto::Field;

/*
 * Every field Settings shows, each with the code that acts on it. A field
 * with no reader is not listed: a switch that changes nothing is a lie.
 * `coverage.rs` holds this list and the screens to each other.
 */
pub const ALL_FIELDS: &[Field] = &[
    Field::Hostname,             // terminal prompt and identity
    Field::Timezone,             // shell menubar clock
    Field::ClockFormat24,        // shell menubar clock
    Field::NotificationsEnabled, // shell notify handler
    Field::WifiRadio,            // this app's scan and join
    Field::SystemKeysGenerated,  // written by the setup wizard
    Field::Wallpaper,            // wallpaper capsule
    Field::MouseSensitivity,     // input router
    Field::Persistent,           // vfs persistence gate
    Field::SoundEnabled,         // shell tones
    Field::Volume,               // shell tones
    Field::AlertSounds,          // shell tones
    Field::KernelPreempt,        // scheduler tick
];
