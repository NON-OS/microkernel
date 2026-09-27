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

use super::section::Section;

pub fn nav(section: Section) -> &'static str {
    text(section).0
}

pub fn title(section: Section) -> &'static str {
    text(section).1
}

pub fn subtitle(section: Section) -> &'static str {
    text(section).2
}

fn text(section: Section) -> (&'static str, &'static str, &'static str) {
    match section {
        Section::General => {
            ("General", "General", "This machine's name, the clock and notifications.")
        }
        Section::Network => {
            ("Network", "Network", "Manage how NONOS connects to networks and the internet.")
        }
        Section::Wifi => ("Wi-Fi", "Wi-Fi", "Find and join a wireless network."),
        Section::Security => {
            ("Security", "Security", "This machine's keys and the protections the kernel keeps on.")
        }
        Section::Appearance => {
            ("Appearance", "Appearance", "The wallpaper and how the pointer moves.")
        }
        Section::Privacy => {
            ("Privacy", "Privacy", "What this machine keeps once it is switched off.")
        }
        Section::Sound => ("Sound", "Sound", "System tones and how loud they are."),
        Section::Storage => {
            ("Storage", "Storage", "How the capsule store and the filesystem are being used.")
        }
        Section::Updates => ("Updates", "Updates", "The signed image this machine is running."),
        Section::Developer => ("Developer", "Developer", "How the scheduler shares the processor."),
    }
}
