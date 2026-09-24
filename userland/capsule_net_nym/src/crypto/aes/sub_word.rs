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

use super::sub_byte::sub_byte;

pub(crate) fn sub_word(word: u32) -> u32 {
    let b = word.to_be_bytes();
    u32::from_be_bytes([
        sub_byte(b[0]),
        sub_byte(b[1]),
        sub_byte(b[2]),
        sub_byte(b[3]),
    ])
}
