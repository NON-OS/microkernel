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

/// The letters after "<" or "</" in escaped script, enough to tell whether
/// they spell "script".
#[derive(Default)]
pub struct Temp {
    buf: [u8; 6],
    len: usize,
}

impl Temp {
    pub(super) fn clear(&mut self) {
        self.len = 0;
    }

    pub(super) fn push(&mut self, c: u8) {
        if let Some(slot) = self.buf.get_mut(self.len) {
            *slot = c.to_ascii_lowercase();
        }
        self.len += 1;
    }

    pub(super) fn is_script(&self) -> bool {
        self.len == 6 && self.buf == *b"script"
    }
}
