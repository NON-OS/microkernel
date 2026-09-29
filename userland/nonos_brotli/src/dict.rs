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

use crate::error::Error;

/// The static dictionary of RFC 7932 appendix A.
static DATA: &[u8; 122_784] = include_bytes!("dictionary.dat");

/// log2 of the number of words of each length (RFC 7932 8).
const NDBITS: [u8; 25] =
    [0, 0, 0, 0, 10, 10, 11, 11, 10, 10, 10, 10, 10, 9, 9, 8, 7, 7, 8, 7, 7, 6, 6, 5, 5];

/// Where the words of each length start in DATA.
static OFFSET: [usize; 25] = offsets();

const fn offsets() -> [usize; 25] {
    let mut at = [0usize; 25];
    let mut len = 4;
    while len < 24 {
        at[len + 1] = at[len] + (len << NDBITS[len]);
        len += 1;
    }
    at
}

/// Number of word transforms (RFC 7932 appendix B).
pub(crate) const TRANSFORMS: usize = 121;

/// The base word and transform a dictionary reference names, given the
/// copy length and how far the distance passes the window.
pub(crate) fn word(len: usize, id: usize) -> Result<(&'static [u8], usize), Error> {
    if !(4..=24).contains(&len) {
        return Err(Error::Invalid);
    }
    let bits = NDBITS[len];
    let (index, transform) = (id & ((1 << bits) - 1), id >> bits);
    if transform >= TRANSFORMS {
        return Err(Error::Invalid);
    }
    let at = OFFSET[len] + index * len;
    Ok((&DATA[at..at + len], transform))
}
