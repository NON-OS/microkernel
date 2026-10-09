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
use core::sync::atomic::{AtomicU32, Ordering};

use nonos_toolkit::decorations::DecorationHit;

use crate::app::{App, AppManifest};
use crate::clients::toolkit;
use crate::setup::WindowBinding;

use super::paint_draw::draw;

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
    hover: DecorationHit,
    maximized: bool,
    toolkit_port: u32,
    request_id: u32,
) {
    let words = (binding.byte_len / 4) as usize;
    let surface: &mut [u32] =
        unsafe { core::slice::from_raw_parts_mut(binding.backing_va as *mut u32, words) };
    let mut back: Vec<u32> = Vec::new();
    if back.try_reserve_exact(words).is_err() {
        draw(app, manifest, binding, surface, hover, maximized);
    } else {
        back.resize(words, 0);
        draw(app, manifest, binding, &mut back, hover, maximized);
        surface.copy_from_slice(&back);
    }
    frame_handshake(toolkit_port, request_id, binding.surface_handle, binding.width, binding.height);
}

/// The last theme revision this process applied; `u32::MAX` until the first
/// frame, so the first handshake always syncs the app to the toolkit's theme.
static APPLIED_REVISION: AtomicU32 = AtomicU32::new(u32::MAX);

/// Tell the toolkit of the frame just drawn and read back its theme revision.
/// The toolkit attaches and paints nothing for this, so it is a cheap per-frame
/// call. When the revision has moved, fetch the theme and apply it in process,
/// so later frames paint with the theme the system now carries.
fn frame_handshake(port: u32, request_id: u32, surface_handle: u64, width: u32, height: u32) {
    let Ok(revision) = toolkit::ui_frame(port, request_id, surface_handle, width, height) else {
        return;
    };
    if revision == APPLIED_REVISION.load(Ordering::Relaxed) {
        return;
    }
    let Ok(theme) = toolkit::theme_get(port, request_id) else {
        return;
    };
    let mut roles = [0u8; 20];
    roles[0..4].copy_from_slice(&theme.background_argb.to_le_bytes());
    roles[4..8].copy_from_slice(&theme.surface_argb.to_le_bytes());
    roles[8..12].copy_from_slice(&theme.accent_argb.to_le_bytes());
    roles[12..16].copy_from_slice(&theme.text_argb.to_le_bytes());
    roles[16..20].copy_from_slice(&theme.border_argb.to_le_bytes());
    let _ = nonos_toolkit::theme::apply(&roles);
    APPLIED_REVISION.store(revision, Ordering::Relaxed);
}
