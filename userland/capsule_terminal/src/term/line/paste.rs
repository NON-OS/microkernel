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

use nonos_app_skeleton::input::text::paste_char;

use super::types::Line;

/// What a paste did to the line.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Pasted {
    /// Something was inserted.
    pub changed: bool,
    /// The line filled before the text ran out.
    pub full: bool,
}

impl Line {
    /// Insert one line of pasted text at the cursor, character by character
    /// as typing would: any printable character, a tab as a space, other
    /// control characters dropped. It stops at the first character that no
    /// longer fits, so the line never ends in part of one.
    pub fn paste(&mut self, text: &str) -> Pasted {
        let mut out = Pasted::default();
        for ch in text.chars().filter_map(paste_char) {
            if !self.insert_char(ch) {
                out.full = true;
                break;
            }
            out.changed = true;
        }
        out
    }
}
