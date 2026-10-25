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

//! A primary's cursor slot as memory the compositor draws the arrow into.

use super::cursor_shape::SLOT;
use crate::sw_blitter::Surface;

/// A primary's cursor slot, when it starts past nothing and ends inside
/// the attachment; a driver's offset is not trusted for either.
pub fn slot(s: Surface, offset: u32) -> Option<&'static mut [u8]> {
    let end = (offset as u64).checked_add(SLOT as u64)?;
    if offset == 0 || end > s.byte_len {
        return None;
    }
    let va = usize::try_from(s.base_va.checked_add(offset as u64)?).ok()?;
    // SAFETY: inside the attachment checked above, which lives as long as
    // the primary it is part of; nothing else here writes the slot.
    Some(unsafe { core::slice::from_raw_parts_mut(va as *mut u8, SLOT) })
}
