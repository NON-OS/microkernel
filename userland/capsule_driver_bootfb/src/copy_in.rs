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

//! A flush into the framebuffer, in its layout.

use super::fb::{Fb, Layout};
use nonos_gfx_serve::{copy_rects, Shadow};
use nonos_gfx_wire::Rect;

impl Fb {
    /// Copy `rects` of `shadow` in, in this framebuffer's layout.
    pub fn copy_in(&self, rects: &[Rect], shadow: &Shadow) {
        let swap = self.layout == Layout::RedFirst;
        // SAFETY: `new` checked the mapping covers the mode, and the
        // driver keeps it mapped while it serves.
        unsafe {
            copy_rects(
                shadow.bytes,
                shadow.stride as usize,
                self.base,
                self.len,
                self.stride as usize,
                rects,
                swap,
            )
        };
    }
}
