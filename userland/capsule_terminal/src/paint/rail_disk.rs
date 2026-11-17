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
use nonos_toolkit::widgets::Surface;

use super::rail_fmt::mib_into;
use super::rail_metric::one;
use super::rail_text::{head, stat};
use crate::rail::disk::Disk;

pub use super::rail_geom::disk_h as height;

/// The mount rows. The VFS reports what its store holds, so Used is measured;
/// nothing reports a byte ceiling to hold it against, so Total stays a standing
/// gap the section says out loud instead of quietly omitting.
pub fn draw(fb: &mut impl Surface, x: u32, y: u32, w: u32, d: &Disk) {
    let mut b = [0u8; 32];
    let mut y = head(fb, x, y, w, "Disk usage");
    y = stat(fb, x, y, w, "Total", one(&mut b, d.total_kb, mib_into));
    stat(fb, x, y, w, "Used", one(&mut b, d.used_kb, mib_into));
}
