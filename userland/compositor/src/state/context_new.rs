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

//! A context on a fresh primary: the scene empty, the whole screen damaged,
//! the cursor centred. Every display path (GOP, the legacy virtio ops, the
//! wire) builds one here, so a field added to the context is set once.

use super::{AttachCache, Context, CursorTracker, DamageAccumulator, FocusTable, SceneTable};

/// The memory the compositor draws into, as the path attached it.
#[derive(Clone, Copy)]
pub struct Primary {
    pub handle: u64,
    pub va: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub byte_len: u64,
}

impl Context {
    /// `gfx_port` 0 with `gop_mode` for GOP; a GOP or wire primary needs
    /// no SET_SCANOUT, so only the legacy path starts with it owed.
    pub fn new(p: Primary, gfx_port: u32, resource_id: u32, gop_mode: bool) -> Self {
        let mut damage = DamageAccumulator::new();
        damage.mark_full(p.width, p.height);
        Context {
            gfx_port,
            resource_id,
            width: p.width,
            height: p.height,
            stride: p.stride,
            backing_len: p.byte_len,
            backing_va: p.va,
            gop_mode,
            surface_handle: p.handle,
            first_scanout_done: gop_mode,
            scanout_error_reported: false,
            next_request_id: 2,
            scene: SceneTable::new(),
            damage,
            focus: FocusTable::new(),
            cursor: CursorTracker::at(p.width / 2, p.height / 2),
            attach: AttachCache::new(),
            scale: 1,
            screen: crate::sw_blitter::Surface::default(),
            wire: None,
        }
    }
}
