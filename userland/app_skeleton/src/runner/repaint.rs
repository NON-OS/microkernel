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

use nonos_toolkit::decorations::FrameState;

use crate::app::App;
use crate::discover::Peers;

use super::boot::BootedApp;
use super::hover_part::hover_part;
use super::paint_frame::{paint, paint_changed};
use super::paint_hover::paint_hover;
use super::paint_partial::paint_partial;

/* Redraw the window. The frame is redrawn only when the window changed size
 * or state or a frame button's hover changed; otherwise the app says which
 * part of its content moved, and only that part is presented, or, when it
 * names none, the rows its whole frame changed (paint_changed). A frame
 * that found no buffer free, or whose present was refused, leaves the
 * window to be painted whole again (`ensure_primed`). */
pub(super) fn repaint<A: App>(booted: &mut BootedApp<A>, peers: &Peers, request_id: &mut u32) {
    let b = &booted.binding;
    let state = FrameState { hover: booted.drag.hover, focused: booted.focused };
    let frame = (b.width, b.height, state, booted.maximized);
    let same = frame == booted.painted;
    let damage = if same { booted.app.take_damage() } else { None };
    booted.painted = frame;
    let shown = match damage {
        Some(rect) => paint_partial(booted, peers, request_id, rect, state),
        None => {
            let BootedApp { app, manifest, binding, maximized, .. } = booted;
            let frame = (state, *maximized);
            match same {
                true => paint_changed(app, manifest, binding, frame, peers, request_id),
                false => paint(app, manifest, binding, frame, peers, request_id),
            }
        }
    };
    if !shown {
        booted.primed = false;
    }
}

/// Only the lit frame control changed (a drain whose one ask was a hover):
/// the controls put out and lit are presented, nothing else of the window
/// (hover_part.rs). Anything more than the hover that differs from the
/// frame painted, the size, the focus or maximized, is drawn as repaint
/// draws it.
pub(super) fn repaint_hover<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
) {
    let b = &booted.binding;
    let state = FrameState { hover: booted.drag.hover, focused: booted.focused };
    let frame = (b.width, b.height, state, booted.maximized);
    let q = super::chrome::quarters();
    let Some(part) = hover_part(booted.painted, frame, q) else {
        return repaint(booted, peers, request_id);
    };
    booted.painted = frame;
    // Neither was a control: the window draws the same as on screen.
    let Some(part) = part else { return };
    if !paint_hover(booted, peers, request_id, part) {
        booted.primed = false;
    }
}
