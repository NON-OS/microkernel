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

//! The metadata section: every key read and its value bounded; four keys
//! kept, because the layout and the model's identity depend on them.

use crate::cursor::Cursor;
use crate::error::GgufError;
use crate::limits::Limits;
use crate::source::ReadAt;
use crate::summary::Meta;
use crate::value::{skip_value, TY_STRING, TY_U32};

/// GGUF's alignment when general.alignment is absent.
pub const DEFAULT_ALIGNMENT: u32 = 32;
const MAX_ALIGNMENT: u32 = 1 << 20;

pub(crate) fn read_meta<S: ReadAt>(
    c: &mut Cursor<'_, S>,
    keys: u64,
    lim: &Limits,
) -> Result<Meta, GgufError> {
    let mut meta =
        Meta { alignment: DEFAULT_ALIGNMENT, architecture: None, name: None, file_type: None };
    for _ in 0..keys {
        let key = c.text(lim.max_string)?;
        let at = c.pos;
        let ty = c.u32()?;
        let want = |t: u32| if ty == t { Ok(()) } else { Err(GgufError::KeyType { at, ty }) };
        match if key.is_whole() { key.as_bytes() } else { &[] } {
            b"general.alignment" => {
                want(TY_U32)?;
                let a = c.u32()?;
                if a == 0 || !a.is_power_of_two() || a > MAX_ALIGNMENT {
                    return Err(GgufError::BadAlignment(a));
                }
                meta.alignment = a;
            }
            b"general.architecture" => {
                want(TY_STRING)?;
                meta.architecture = Some(c.text(lim.max_string)?);
            }
            b"general.name" => {
                want(TY_STRING)?;
                meta.name = Some(c.text(lim.max_string)?);
            }
            b"general.file_type" => {
                want(TY_U32)?;
                meta.file_type = Some(c.u32()?);
            }
            _ => skip_value(c, ty, at, lim)?,
        }
    }
    Ok(meta)
}
