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

use crate::dict::word;
use crate::error::Error;
use crate::transform::{apply, Word};
use alloc::vec::Vec;

/// Append the word a distance past the window names; `dist` is that
/// distance and `limit` the window. A word must fit the meta-block, and
/// one that comes out empty may not be coded without extra bits.
pub(crate) fn emit(
    out: &mut Vec<u8>,
    len: usize,
    (dist, limit): (usize, usize),
    end: usize,
) -> Result<(), Error> {
    let (base, id) = word(len, dist - limit - 1)?;
    let mut buf: Word = [0; 40];
    let n = apply(base, id, &mut buf);
    if n > end - out.len() || (n == 0 && dist <= 120) {
        return Err(Error::Invalid);
    }
    out.extend_from_slice(&buf[..n]);
    Ok(())
}
