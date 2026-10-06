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

use super::walk::{walk, Walk};

/* True once the last chunk and the trailer section have arrived. */
pub fn complete(body: &[u8]) -> bool {
    matches!(walk(body, None), Walk::Done(_))
}

/* Bytes the complete chunked body spans, trailer section included; on a
kept-alive connection the next response starts there. */
pub fn frame_end(body: &[u8]) -> Option<usize> {
    match walk(body, None) {
        Walk::Done(n) => Some(n),
        _ => None,
    }
}
