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

//! The box a client said changed of a buffer shown at another size (a
//! viewport, the display's ratio), resampled at the size shown into the
//! window's reused scratch, then laid into the window.

use nonos_toolkit::decorations::frame_bg;

use super::super::super::super::present::draw::{draw, Drew};
use super::super::super::super::present::frame::geometry::Around;
use super::super::super::super::present::source::Source;
use super::super::super::super::scene::Scene;
use super::super::damage::Damage;
use super::{counted, damage_io, damage_lay, show_box};

/// The bytes read showing `said` of `src` on a window with frame `a`, and
/// what the draw came to; None when it is shown whole.
pub fn part(
    pid: u32,
    scene: &mut Scene,
    src: &Source,
    a: Around,
    said: Damage,
) -> Option<(u64, Drew)> {
    // The changed rows land in the window's reused scratch, taken out of the
    // frame while it is read and laid, then put back with its capacity kept.
    let mut rows = core::mem::take(&mut scene.win.frame.scratch);
    let (x0, y0, x1, y1) = match damage_io::changed(pid, src, said, &mut rows) {
        Some(Some(b)) => b,
        got => {
            scene.win.frame.scratch = rows;
            return got.map(|_| (0, Drew::Nothing));
        }
    };
    let read = rows.len() as u64;
    let stride = src.shape.2;
    let (box_, put) = ((x0, 0, x1, y1 - y0), (stride, 0, y0));
    if !a.framed() {
        let changed = (x0, y0, x1 - x0, y1 - y0);
        let ground = Some(frame_bg());
        let drew = draw(scene, src.bytes, changed, |_, out| {
            counted(damage_lay::lay(out, put, &rows, stride, box_, ground), box_).then_some(changed)
        });
        scene.win.frame.scratch = rows;
        return Some((read, drew));
    }
    if !scene.win.frame.holds(src.shape) {
        scene.win.frame.scratch = rows;
        return None;
    }
    let Some(client) = scene.win.frame.client_room(src.bytes) else {
        scene.win.frame.scratch = rows;
        return None;
    };
    if !counted(damage_lay::lay(client, put, &rows, stride, box_, None), box_) {
        // Some rows laid, not all: no longer a whole picture.
        scene.win.frame.client_shape = None;
        scene.win.frame.scratch = rows;
        return None;
    }
    // The box is in `client` now; the scratch goes back for the next frame.
    scene.win.frame.scratch = rows;
    Some((read, show_box(scene, a, stride, (x0, y0, x1, y1))))
}
