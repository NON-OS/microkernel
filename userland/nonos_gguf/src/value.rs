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

//! Metadata values, stepped over by type without being kept.

use crate::cursor::Cursor;
use crate::error::GgufError;
use crate::limits::Limits;
use crate::source::ReadAt;
use crate::value_array::skip_array;

pub(crate) const TY_U32: u32 = 4;
pub(crate) const TY_STRING: u32 = 8;
pub(crate) const TY_ARRAY: u32 = 9;

/// Bytes of one value of a fixed-size type, or None for strings, arrays and
/// types GGUF does not define.
pub(crate) fn fixed_size(ty: u32) -> Option<u64> {
    match ty {
        0 | 1 | 7 => Some(1),
        2 | 3 => Some(2),
        4..=6 => Some(4),
        10..=12 => Some(8),
        _ => None,
    }
}

/// Step over one value of type `ty` whose type field began at `at`.
pub(crate) fn skip_value<S: ReadAt>(
    c: &mut Cursor<'_, S>,
    ty: u32,
    at: u64,
    lim: &Limits,
) -> Result<(), GgufError> {
    if let Some(n) = fixed_size(ty) {
        return c.skip(n);
    }
    match ty {
        TY_STRING => c.skip_string(lim.max_string),
        TY_ARRAY => skip_array(c, lim),
        _ => Err(GgufError::UnknownValueType { at, ty }),
    }
}
