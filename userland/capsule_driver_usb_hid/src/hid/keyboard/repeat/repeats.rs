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

//! Which keys repeat when held.

/// Whether a held usage repeats: every key but the locks, Print Screen,
/// Pause, Power and Mute, which act once per press. Mute held would flip the
/// sound at the repeat rate; Volume Up and Down repeat, a step each.
pub fn repeats(key: u8) -> bool {
    matches!(key, 0x04..=0xA4) && !matches!(key, 0x39 | 0x46 | 0x47 | 0x48 | 0x53 | 0x66 | 0x7f)
}
