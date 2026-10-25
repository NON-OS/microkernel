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
//! the cursor centred. Both display paths (GOP and the wire) build one
//! here, so a field added to the context is set once.

use super::primary::Primary;
use super::{AttachCache, Context, CursorTracker, DamageAccumulator, FocusTable, SceneTable};

impl Context {
    /// `gfx_port` 0 with `gop_mode` for GOP.
    pub fn new(p: Primary, gfx_port: u32, gop_mode: bool) -> Self {
        let mut damage = DamageAccumulator::new();
        damage.mark_full(p.width, p.height);
        Context {
            gfx_port,
            width: p.width,
            height: p.height,
            stride: p.stride,
            backing_len: p.byte_len,
            backing_va: p.va,
            gop_mode,
            gop_held: false,
            surface_handle: p.handle,
            scanout_error_reported: false,
            scene: SceneTable::new(),
            damage,
            focus: FocusTable::new(),
            cursor: CursorTracker::at(p.width / 2, p.height / 2),
            attach: AttachCache::new(),
            scale_q: 4,
            output_scale: [None; 4],
            screen: crate::sw_blitter::Surface::default(),
            wire: None,
            heads: alloc::vec::Vec::new(),
            cursor_sent: None,
            resize: crate::display::resize_rule::Resize::default(),
            indicators: super::indicator::Indicators::default(),
            want_extend: false,
            plan: None,
            head_pending: alloc::vec::Vec::new(),
            screen_notice_until: 0,
            looks: super::looks::Looks::default(),
            first_frame_since: None,
            lid: super::lid_rule::Lid::Open,
            dma_unconfined: crate::setup::dma::unconfined(),
        }
    }
}
