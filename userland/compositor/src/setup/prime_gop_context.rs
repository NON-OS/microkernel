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

//! The compositor's state on the GOP path, drawing straight into its own
//! primary surface.

use crate::state::{
    AttachCache, Context, CursorTracker, DamageAccumulator, FocusTable, SceneTable,
};

pub(super) fn gop_context(
    width: u32,
    height: u32,
    stride: u32,
    byte_len: u64,
    va: u64,
    handle: u64,
) -> Context {
    let mut damage = DamageAccumulator::new();
    damage.mark_full(width, height);
    Context {
        gfx_port: 0,
        resource_id: 0,
        width,
        height,
        stride,
        backing_len: byte_len,
        backing_va: va,
        gop_mode: true,
        surface_handle: handle,
        first_scanout_done: true,
        scanout_error_reported: false,
        next_request_id: 2,
        scene: SceneTable::new(),
        damage,
        focus: FocusTable::new(),
        cursor: CursorTracker::at(width / 2, height / 2),
        attach: AttachCache::new(),
        scale: 1,
        screen: crate::sw_blitter::Surface::default(),
    }
}
