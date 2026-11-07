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

//! The frame painted again when only it changed: a control lit, a title.

use nonos_toolkit::decorations::DecorationHit;

use super::super::super::scene::Scene;
use super::super::super::scene_buffers::Patience;
use super::super::draw::{draw_as, whole, Drew};
use super::now::shown;
use super::paint::{paint, Look};

/// The longest title kept, in bytes.
const TITLE_MAX: usize = 160;

/// The frame painted whole again, the guest's pixels it keeps laid in it.
/// Also what a window owed a repaint is drawn with (owed.rs).
pub fn redraw(scene: &mut Scene) -> Drew {
    redraw_as(scene, Patience::Fence, Told::Whole)
}

/// What a frame painted again is presented as.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Told {
    Whole,
    /// The band above the content, which holds the title bar's controls
    /// and title: a control lit or a title set changes no pixel below it.
    Bar,
}

fn redraw_as(scene: &mut Scene, patience: Patience, told: Told) -> Drew {
    let Some((a, shape)) = shown(scene) else { return Drew::Nothing };
    // The kept pixels are laid at the window's shape only when they are of
    // it; else the next commit draws the window whole (state.rs).
    if !a.framed() || !scene.win.frame.holds(shape) {
        return Drew::Nothing;
    }
    // A frame of another shape than the buffers are laid at (a configure
    // acked, its buffer not committed yet) is drawn by that commit, which
    // takes new buffers (show.rs): drawn here it would shear.
    if scene.win.bufs.shape() != a.laid(shape) {
        return Drew::Nothing;
    }
    let all = whole((a.w, a.h));
    // Painted whole all the same, so the buffer is the whole picture; only
    // what it presents changes. A window owed a repaint may show a picture
    // that is not its own below the bar, so it is presented whole.
    let changed = match (told, scene.win.owing.is_some()) {
        (Told::Bar, false) => (0, 0, a.w, a.cy),
        _ => all,
    };
    let bytes = (a.w as usize) * (a.h as usize) * 4;
    let drew = draw_as(scene, bytes, all, patience, |scene, out| {
        let frame = &scene.win.frame;
        let look = Look { title: &frame.title, hover: frame.hover, focused: scene.win.focused };
        let painted = paint(out, a, &frame.client, shape, (&look, true), frame.quarters);
        painted.then_some(changed)
    });
    if drew == Drew::Shown {
        scene.win.frame.chrome = Some((a, scene.win.frame.hover, scene.win.focused));
        super::super::owed::paid_repaint(&mut scene.win);
    }
    drew
}

/// The control under the pointer is now `hit`: the frame again if it
/// changed. The pointer moved to make it, so the draw waits for no buffer:
/// none free, the window is owed the repaint (owed.rs) and the input behind
/// this motion is not held up to two refreshes for a highlight.
pub fn light(scene: &mut Scene, hit: DecorationHit) {
    if scene.win.frame.hover != hit {
        scene.win.frame.hover = hit;
        redraw_as(scene, Patience::Now, Told::Bar);
    }
}

/// The guest named its window: the title bar says so.
pub fn title(scene: &mut Scene, title: &[u8]) {
    let title = trimmed(title);
    if scene.win.frame.title != title {
        scene.win.frame.title.clear();
        scene.win.frame.title.extend_from_slice(title);
        redraw_as(scene, Patience::Fence, Told::Bar);
    }
}

/// A title as a title bar shows it: whole UTF-8 only, and no longer than a
/// title bar could show.
pub fn trimmed(title: &[u8]) -> &[u8] {
    let title = title.strip_suffix(b"\0").unwrap_or(title);
    let mut end = title.len().min(TITLE_MAX);
    while end > 0 && core::str::from_utf8(&title[..end]).is_err() {
        end -= 1;
    }
    &title[..end]
}
