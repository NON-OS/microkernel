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

use super::types::Keyboard;

impl Keyboard {
    /// Called from the poll loop: press the held key again when its repeat is
    /// due. It resolves under the modifiers held now, so Shift pressed during
    /// a repeat types capitals from then on. Ctrl+Alt held over a repeating
    /// Space ends the repeat instead of cycling the layout thirty times a
    /// second.
    pub fn tick(&mut self, now_ms: u64) {
        let Some(key) = self.repeat.due(now_ms) else { return };
        let chord = key == 0x2c && self.modifiers & 0x11 != 0 && self.modifiers & 0x04 != 0;
        if chord || self.held.code(u32::from(key)) == Some(0) {
            self.repeat.release(key);
            return;
        }
        self.post(key, true);
    }
}
