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

pub fn note(field: Field) -> Option<&'static str> {
    Some(match field {
        Field::NotificationsEnabled => "News from apps. Warnings and errors always show.",
        Field::WifiRadio => "Scan for and join wireless networks.",
        Field::Timezone => "Hours from UTC, used by the menu bar clock.",
        Field::MouseSensitivity => "Scales mouse movement only.",
        Field::Persistent => "Files and installed apps are kept between boots.",
        // The desktop's tones are all these reach (shell sound/levels.rs):
        // Music keeps a volume of its own, and the rows said otherwise.
        Field::SoundEnabled => "The desktop's own tones. Music has its own volume.",
        Field::Volume => "How loud the desktop's tones are. Music has its own volume.",
        Field::AlertSounds => "A tone for warnings and errors.",
        _ => return None,
    })
}
