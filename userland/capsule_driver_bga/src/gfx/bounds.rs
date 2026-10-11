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

//! Whether a rect a present names lies where the copy may touch it.

use super::screen::{Rect, Screen};
use crate::protocol::E_INVAL;

impl Screen {
    pub fn inside(&self, r: Rect) -> Result<(), i32> {
        let fits = r.w != 0
            && r.h != 0
            && r.x.checked_add(r.w).is_some_and(|e| e <= self.mode.width)
            && r.y.checked_add(r.h).is_some_and(|e| e <= self.mode.height);
        if fits {
            Ok(())
        } else {
            Err(E_INVAL)
        }
    }

    /// The rect's rows in the shadow start at `offset`, a pixel boundary,
    /// one stride apart, and end inside it.
    pub fn source(&self, r: Rect, offset: u64) -> Result<(), i32> {
        let stride = self.mode.stride as u64;
        let end = (r.h as u64 - 1)
            .checked_mul(stride)
            .and_then(|rows| rows.checked_add(r.w as u64 * 4))
            .and_then(|span| span.checked_add(offset));
        match end {
            Some(e) if offset.is_multiple_of(4) && e <= self.mode.bytes() => Ok(()),
            _ => Err(E_INVAL),
        }
    }

    /// SET_SCANOUT: scanout 0 shows the whole resource; the mode is fixed.
    pub fn whole(&self, scanout: u32, r: Rect) -> Result<(), i32> {
        let all = Rect { x: 0, y: 0, w: self.mode.width, h: self.mode.height };
        if scanout == 0 && r == all {
            Ok(())
        } else {
            Err(E_INVAL)
        }
    }
}
