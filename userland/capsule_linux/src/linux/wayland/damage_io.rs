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

//! A window's changed rows read from its guest, and the box they changed
//! damaged where the window is on screen.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::compositor::damage_commit;
use nonos_app_skeleton::discover::lookup_port;

use crate::linux::guest::Guest;

use super::super::super::super::opaque::make_opaque;
use super::super::super::super::present::source::Source;
use super::super::super::super::scene::Scene;

#[path = "damage_changed.rs"]
mod in_box;
pub use in_box::changed;

/// Rows `y0` up to `y1` of `src` read from guest `pid`, opaque when it is
/// XRGB. Outside the box it said changed a client's buffer may hold an
/// older frame, so the rows are read apart and only the box is laid.
pub fn rows(pid: u32, src: &Source, (y0, y1): (u32, u32)) -> Option<Vec<u8>> {
    let (w, _, stride) = src.shape;
    let from = y0 as usize * stride as usize;
    let to = (y1 as usize * stride as usize).min(src.bytes);
    let mut rows = Vec::new();
    rows.try_reserve_exact(to.checked_sub(from)?).ok()?;
    rows.resize(to - from, 0);
    if !Guest::read_into(pid, src.at + from as u64, &mut rows) {
        return None;
    }
    if src.opaque {
        make_opaque(&mut rows, w, y1 - y0, stride);
    }
    Some(rows)
}

/// The box `r` of the content, at (`cx`, `cy`) in the window, damaged.
pub fn tell(scene: &mut Scene, (cx, cy): (u32, u32), r: (u32, u32, u32, u32)) {
    let (Some((x, y)), Some(port)) = (scene.at, lookup_port(b"compositor")) else { return };
    if r.2 > r.0 && r.3 > r.1 {
        let (x, y, w, h) = (x + cx + r.0, y + cy + r.1, r.2 - r.0, r.3 - r.1);
        let _ = damage_commit(port, scene.next_serial(), x, y, w, h);
    }
}
