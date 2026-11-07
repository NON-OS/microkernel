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

pub const OP_HEALTHCHECK: u16 = 0x0001;
pub const OP_SCENE_SUBMIT: u16 = 0x0002;
pub const OP_DAMAGE_COMMIT: u16 = 0x0003;
pub const OP_FOCUS_SET: u16 = 0x0004;
pub const OP_INPUT_SUBSCRIBE: u16 = 0x0005;
pub const OP_CURSOR_UPDATE: u16 = 0x0006;
pub const OP_SCENE_REMOVE: u16 = 0x0007;
pub const OP_DISPLAY_INFO: u16 = 0x0008;
/// A guest service's host size hint (section 12, Size).
pub const OP_DISPLAY_HINT: u16 = 0x0009;
/// A capturing driver's camera or microphone on or off.
pub const OP_INDICATOR: u16 = 0x000A;
/// The shell's choice for several outputs: mirrored or extended (unit 10).
pub const OP_OUTPUT_LAYOUT: u16 = 0x000B;
/// The composed screen into a capture holder's surface, with a notice.
pub const OP_SCREENSHOT: u16 = 0x000C;
/// The shell's scale for one output, in quarters.
pub const OP_OUTPUT_SCALE: u16 = 0x000D;
/// The shell's colour temperature for the screen (night light).
pub const OP_NIGHT_LIGHT: u16 = 0x000E;
/// The shell's accessibility settings for one output.
pub const OP_ACCESSIBILITY: u16 = 0x000F;
/// One output's accessibility settings as they are now.
pub const OP_ACCESSIBILITY_GET: u16 = 0x0010;
/// The displays as laid out now, for Settings through the shell.
pub const OP_OUTPUT_ARRANGEMENT: u16 = 0x0011;
/// A display driver's word, unanswered (`nonos_gfx_wire::notice`).
pub const OP_OUTPUT_NOTICE: u16 = 0x0012;
/// The shell's lid: closed darkens every output, open wakes them.
pub const OP_LID: u16 = 0x0013;
/// A brightness key: the focused output's panel to its next level.
pub const OP_BRIGHTNESS_STEP: u16 = 0x0014;
/// One of a monitor's own controls set, on the person's word.
pub const OP_SET_VCP: u16 = 0x0015;
pub const OP_BRIGHTNESS_GET: u16 = 0x0016;
pub const OP_INPUT_SOURCES: u16 = 0x0017;
/// The pointer's shape over one of the caller's own surfaces (a resize
/// arrow over a window's edge), drawn while the pointer is over it.
pub const OP_CURSOR_SHAPE: u16 = 0x0018;
/// The shell turns per-stage frame timing on (1) or off (0).
pub const OP_FRAME_TRACE: u16 = 0x0019;
/// A swapchain client's layer shows another of its buffers, with the rect
/// that changed (nonos_surface::Present).
pub const OP_PRESENT: u16 = 0x001A;
/// When the screen last changed: frames shown and the time of the last
/// (state/presented.rs). Empty body.
pub const OP_FRAME_TIME: u16 = 0x001B;
/// A layer named by its surface handle: an owner may hold several in one
/// band, one per window (requests/place_keyed.rs).
pub const OP_SCENE_SUBMIT_LAYER: u16 = 0x001C;
/// Drop one of the caller's keyed layers, by band and handle.
pub const OP_SCENE_REMOVE_LAYER: u16 = 0x001D;
/// The window manager raised one window: its layers go on top of their bands.
pub const OP_FOCUS_SET_WINDOW: u16 = 0x001E;
/// The input router's idle blank: 1 darkens every output as a closed lid
/// does, 0 wakes them (unless the lid holds them dark).
pub const OP_IDLE_BLANK: u16 = 0x001F;
/// The display driver a swapchain client may let read its buffers, so a
/// full-screen opaque one can be scanned out directly: its pid as a u32,
/// 0 when the driver cannot. Empty body, any client.
pub const OP_SCANOUT_PEER: u16 = 0x0020;
/// From the shell only: each output's place on the canvas.
pub const OP_OUTPUT_RECTS: u16 = 0x0021;
/// A swapchain present that names its layer by the buffer the layer shows
/// now: PRESENT's 32 bytes, then that handle as a u64. For a client with
/// several layers in one band, each a window of its own.
pub const OP_PRESENT_LAYER: u16 = 0x0022;
/// The sender's layer showing a handle draws another surface over its own,
/// from a corner inside it: a client's buffer shown where it drew it, under
/// the frame the layer's own surface holds. Handle 0 draws none again.
pub const OP_LAYER_CONTENT: u16 = 0x0023;
/// OP_PRESENT_LAYER, its changed box told apart as up to four boxes inside
/// it, each repainted alone (requests/parts.rs).
pub const OP_PRESENT_LAYER_PARTS: u16 = 0x0024;
/// OP_LAYER_CONTENT, its changed box told apart the same way.
pub const OP_LAYER_CONTENT_PARTS: u16 = 0x0025;
