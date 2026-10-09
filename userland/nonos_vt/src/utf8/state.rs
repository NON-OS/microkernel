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

//! The decoder's state between bytes.

#[derive(Clone, Copy, Default, Debug)]
pub struct Utf8 {
    pub(super) cp: u32,
    pub(super) need: u8,
    pub(super) min: u32,
}

pub enum Step {
    /// More bytes are needed.
    Pending,
    Char(char),
    /// The sequence was malformed. When `again` is set, the byte that broke
    /// it starts something new and must be fed again.
    Invalid {
        again: bool,
    },
}

impl Utf8 {
    pub fn pending(&self) -> bool {
        self.need > 0
    }

    pub fn reset(&mut self) {
        *self = Utf8::default();
    }
}
