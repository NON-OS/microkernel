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
/// A capturing driver's camera or microphone on or off (Decision 9).
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
