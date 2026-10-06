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

use alloc::vec::Vec;

use super::walk::{walk, Walk};

/* The body of a complete chunked message, or None while it is cut short
or when its framing is malformed. */
pub fn decode(body: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    matches!(walk(body, Some(&mut out)), Walk::Done(_)).then_some(out)
}

/* The data received so far and whether the body is complete; None when
the framing is malformed, which no further byte can repair. */
pub fn decode_partial(body: &[u8]) -> Option<(Vec<u8>, bool)> {
    let mut out = Vec::new();
    match walk(body, Some(&mut out)) {
        Walk::Done(_) => Some((out, true)),
        Walk::Short => Some((out, false)),
        Walk::Bad => None,
    }
}
