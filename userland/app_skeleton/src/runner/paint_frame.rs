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

use alloc::vec::Vec;

use nonos_toolkit::decorations::FrameState;

use crate::app::{App, AppManifest};
use crate::clients::toolkit;
use crate::setup::WindowBinding;

use super::paint_draw::draw;
use super::theme_handshake::follow_theme;

/*
 * The compositor reads the shared surface whenever it composites, on another
 * CPU while this one paints. A frame drawn in place is cleared to transparent
 * and then built up row by row, so a composite in between showed the desktop
 * through the window, or only its top rows: with several CPUs about every
 * other presented frame lost the window while an app repainted busily. Each
 * frame is drawn in a private buffer and copied over the surface in one pass,
 * so the compositor only ever sees a whole frame, old or new. When the heap
 * cannot spare the buffer the frame is drawn in place as before.
 */
pub(super) fn paint<A: App>(
    app: &mut A,
    manifest: &AppManifest,
    binding: &WindowBinding,
    frame: FrameState,
    maximized: bool,
    toolkit_port: u32,
    request_id: u32,
) {
    let words = (binding.byte_len / 4) as usize;
    let surface: &mut [u32] =
        unsafe { core::slice::from_raw_parts_mut(binding.backing_va as *mut u32, words) };
    let mut back: Vec<u32> = Vec::new();
    if back.try_reserve_exact(words).is_err() {
        draw(app, manifest, binding, surface, frame, maximized);
    } else {
        back.resize(words, 0);
        draw(app, manifest, binding, &mut back, frame, maximized);
        surface.copy_from_slice(&back);
    }
    // Tell the toolkit of the frame just drawn and read back its theme
    // revision. It attaches and paints nothing for this: a cheap per-frame call.
    let (handle, w, h) = (binding.surface_handle, binding.width, binding.height);
    if let Ok(revision) = toolkit::ui_frame(toolkit_port, request_id, handle, w, h) {
        follow_theme(toolkit_port, request_id, revision);
    }
}
