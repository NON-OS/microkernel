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

//! The panel's native mode and physical size as the loader has always
//! read them from the base block alone, so moving the loader onto this
//! crate leaves its EDID log lines as they were.

use super::dtd::Detailed;
use crate::edid::Edid;

/// The base block's first detailed timing and the image size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Native {
    pub width: u32,
    pub height: u32,
    /// Millimetres, both 0 when the panel gives no size.
    pub width_mm: u32,
    pub height_mm: u32,
}

impl Edid<'_> {
    /// The native mode. EDID 1.3 and later make the first detailed
    /// timing the preferred one (E-EDID 1.4 section 3.10.1); display
    /// descriptors before it are skipped, but a first timing with a clock
    /// and no picture is no native mode, as the loader has always held.
    pub fn native(&self) -> Option<Native> {
        let d = self.first_slot().and_then(Detailed::fields)?;
        if d.h_active == 0 || d.v_active == 0 {
            return None;
        }
        let (width_mm, height_mm) = self.size_with(d.width_mm, d.height_mm);
        Some(Native { width: d.width(), height: d.height(), width_mm, height_mm })
    }

    /// The image size in millimetres: the native timing's own, else the
    /// base block's bytes 21-22 in centimetres (section 3.6.2). Either of
    /// those two being zero means they hold an aspect ratio, not a size.
    pub fn size_mm(&self) -> (u32, u32) {
        match self.native() {
            Some(n) => (n.width_mm, n.height_mm),
            None => self.size_with(0, 0),
        }
    }

    fn size_with(&self, w_mm: u32, h_mm: u32) -> (u32, u32) {
        if w_mm != 0 && h_mm != 0 {
            return (w_mm, h_mm);
        }
        let b = self.base();
        match (b[21] as u32, b[22] as u32) {
            (0, _) | (_, 0) => (0, 0),
            (w, h) => (w * 10, h * 10),
        }
    }
}
