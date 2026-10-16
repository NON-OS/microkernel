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

use core::ptr::write_volatile;

use super::span::Span;

pub fn clear(fb: &Span, color: u32) {
    let base = fb.va as *mut u32;
    for i in 0..fb.pixels {
        // SAFETY: `span` made `fb` only for pixels that fit the mapping at
        // `va`, so pixel `i` below `pixels` lies inside it.
        unsafe { write_volatile(base.add(i as usize), color) };
    }
}
