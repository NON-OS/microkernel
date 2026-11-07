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

use crate::app::{App, AppManifest};
use crate::clients::toolkit;
use crate::discover::Peers;
use crate::setup::WindowBinding;
use crate::swap::Damage;

use super::changed_band::{present_rows, Shown};
use super::paint_draw::draw;
use super::present_frame::{present, present_parts, ACQUIRE_MS};
use super::request_id::next;
use super::theme_handshake::follow_theme;

/*
 * The compositor reads the buffer a window's layer shows whenever it
 * composites, on another CPU while this one paints. A frame is drawn into
 * a buffer the compositor released (the swapchain, `crate::swap`) and
 * presented once whole, so a composite only ever sees a whole frame, old
 * or new, and the frame is drawn in place with no copy.
 */

/// The whole window drawn into buffer `i`.
pub(super) fn draw_whole<A: App>(
    app: &mut A,
    manifest: &AppManifest,
    binding: &mut WindowBinding,
    i: usize,
    frame: (FrameState, bool),
) {
    let dims = (binding.stride_words, binding.width, binding.height);
    draw(app, manifest, dims, binding.pixels(i), frame.0, frame.1);
}

/// A whole frame, presented whole. False when no buffer came free in time
/// or the compositor refused it: the caller paints again later.
pub(super) fn paint<A: App>(
    app: &mut A,
    manifest: &AppManifest,
    binding: &mut WindowBinding,
    frame: (FrameState, bool),
    peers: &Peers,
    request_id: &mut u32,
) -> bool {
    let Some((i, _)) = binding.swap.acquire(ACQUIRE_MS) else { return false };
    draw_whole(app, manifest, binding, i, frame);
    let whole = Damage { x: 0, y: 0, width: binding.width, height: binding.height };
    if !present(binding, peers.compositor, next(request_id), i, whole) {
        return false;
    }
    tell_toolkit(binding, peers.toolkit, next(request_id));
    true
}

/// A whole frame presented as the rows it changed from the frame on screen,
/// one band or the runs inside it (changed_band.rs present_rows), and not
/// at all when it changed none. Whole while no buffer has been shown. The
/// buffer shown holds what is on screen at the window's size: a present is
/// drawn over all its buffer was out of date by (swap_paint.rs), and a
/// resize opens new buffers drawn whole before the layer takes one. False
/// as `paint`.
pub(super) fn paint_changed<A: App>(
    app: &mut A,
    manifest: &AppManifest,
    binding: &mut WindowBinding,
    frame: (FrameState, bool),
    peers: &Peers,
    request_id: &mut u32,
) -> bool {
    let Some((i, _)) = binding.swap.acquire(ACQUIRE_MS) else { return false };
    draw_whole(app, manifest, binding, i, frame);
    let (width, height) = (binding.width, binding.height);
    let shown = match binding.swap.shown().filter(|&j| j != i) {
        Some(j) => {
            let rows = (binding.stride_words as usize, height as usize);
            present_rows(binding.read(i), binding.read(j), rows, (width, height))
        }
        None => Shown::Band(Damage { x: 0, y: 0, width, height }),
    };
    let (port, rid) = (peers.compositor, next(request_id));
    let presented = match shown {
        Shown::Nothing => return binding.swap.cancel(i),
        Shown::Band(d) => present(binding, port, rid, i, d),
        Shown::Parts(d, parts, n) => present_parts(binding, port, rid, (i, d), &parts[..n]),
    };
    if !presented {
        return false;
    }
    tell_toolkit(binding, peers.toolkit, next(request_id));
    true
}

/// Tell the toolkit of the frame just shown and read back its theme
/// revision. It attaches and paints nothing for this: a cheap per-frame call.
pub(super) fn tell_toolkit(binding: &WindowBinding, port: u32, request_id: u32) {
    let (handle, w, h) = (binding.surface_handle(), binding.width, binding.height);
    if let Ok(revision) = toolkit::ui_frame(port, request_id, handle, w, h) {
        follow_theme(port, request_id, revision);
    }
}
