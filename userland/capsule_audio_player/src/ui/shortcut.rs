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

//! The player's keys, away from the Search field, where typed letters are the
//! query. Space or Enter plays and pauses, and the arrows seek, as before.

use nonos_app_skeleton::{KEY_DOWN, KEY_UP};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shortcut {
    VolumeUp,
    VolumeDown,
    Mute,
    Next,
    Prev,
    Shuffle,
    Repeat,
    /// Open Search, ready for words or an MP3 link.
    Search,
}

/// The shortcut `code` is, if any. Letters in either case.
pub fn shortcut(code: u32) -> Option<Shortcut> {
    let lower = match code {
        0x41..=0x5A => code + 0x20,
        c => c,
    };
    Some(match lower {
        KEY_UP => Shortcut::VolumeUp,
        KEY_DOWN => Shortcut::VolumeDown,
        0x6D => Shortcut::Mute,
        0x6E => Shortcut::Next,
        0x70 => Shortcut::Prev,
        0x73 => Shortcut::Shuffle,
        0x72 => Shortcut::Repeat,
        0x2F => Shortcut::Search,
        _ => return None,
    })
}
