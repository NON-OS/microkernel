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

use super::{AttachCache, CursorTracker, DamageAccumulator, FocusTable, SceneTable};
use crate::sw_blitter::Surface;

pub struct Context {
    pub gfx_port: u32,
    pub resource_id: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub backing_len: u64,
    pub backing_va: u64,
    // GOP mode presents through MkSurfacePresent (kernel blit to the UEFI
    // framebuffer) instead of the virtio-gpu transfer/scanout/flush ops.
    pub gop_mode: bool,
    /// A GOP present was refused because a driver holds the framebuffer.
    pub gop_held: bool,
    pub surface_handle: u64,
    pub first_scanout_done: bool,
    pub scanout_error_reported: bool,
    pub next_request_id: u32,
    pub scene: SceneTable,
    pub damage: DamageAccumulator,
    pub focus: FocusTable,
    pub cursor: CursorTracker,
    pub attach: AttachCache,
    /// Screen pixels per canvas pixel, each way. `width`, `height`, `stride`
    /// and the backing above are the canvas clients draw for; `screen` is
    /// what is presented, the canvas itself when this is one.
    /// Quarters of a screen pixel per canvas pixel on output 0 (4 one to
    /// one, 6 one and a half), and the shell's choice per output.
    pub scale_q: u32,
    pub output_scale: [Option<u32>; 4],
    pub screen: Surface,
    /// The driver when it is driven on the display wire; None on GOP and
    /// on the legacy ops.
    pub wire: Option<crate::display::WireLink>,
    /// The wire driver's outputs past 0, each mirroring the screen.
    pub heads: alloc::vec::Vec<crate::display::Head>,
    /// The pointer as the device's cursor plane last showed it; None owes
    /// the primary a drawn slot and NEW_SHAPE.
    pub cursor_sent: Option<super::cursor::CursorState>,
    /// The host's wanted size and what was asked for (G 6).
    pub resize: crate::display::resize_rule::Resize,
    /// Who has the camera or the microphone on (the capture indicator).
    pub indicators: super::indicator::Indicators,
    /// The shell's choice for several outputs, and the extended canvas's
    /// layout when one is shown (None mirrors).
    pub want_extend: bool,
    pub plan: Option<crate::display::layout_rule::Plan>,
    /// What this frame drew on each further output, flushed once a frame.
    pub head_pending: alloc::vec::Vec<(usize, super::damage::Rect)>,
    /// Until when (uptime ms) the band shows that the screen was captured.
    pub screen_notice_until: i64,
    /// Night light's ramps while the shell has it on (None: true colour).
    pub night: Option<alloc::boxed::Box<[[u8; 256]; 3]>>,
    /// When the first frame's hold began; None once it was let go.
    pub first_frame_since: Option<i64>,
}
