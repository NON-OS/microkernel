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

//! A metadata array: its count bounded, its items stepped over. Arrays of
//! arrays are refused, so the reader never recurses on the file's say-so.

use crate::cursor::Cursor;
use crate::error::GgufError;
use crate::limits::Limits;
use crate::source::ReadAt;
use crate::value::{fixed_size, TY_ARRAY, TY_STRING};

pub(crate) fn skip_array<S: ReadAt>(c: &mut Cursor<'_, S>, lim: &Limits) -> Result<(), GgufError> {
    let at = c.pos;
    let item = c.u32()?;
    let count = c.u64()?;
    if count > lim.max_array {
        return Err(GgufError::ArrayTooLong { at, len: count });
    }
    if let Some(n) = fixed_size(item) {
        /* count <= 2^24 and n <= 8, so the product cannot overflow. */
        return c.skip(count * n);
    }
    match item {
        TY_STRING => {
            for _ in 0..count {
                c.skip_string(lim.max_string)?;
            }
            Ok(())
        }
        TY_ARRAY => Err(GgufError::NestedArray { at }),
        _ => Err(GgufError::UnknownValueType { at, ty: item }),
    }
}
