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

pub const IPC_PAYLOAD_MAX: usize = 256;
pub const STATUS_LEN: usize = 4;
pub const SCENE_SUBMIT_REQ_LEN: usize = 32;
pub const DAMAGE_COMMIT_REQ_LEN: usize = 16;
pub const FOCUS_SET_REQ_LEN: usize = 8;
pub const CURSOR_UPDATE_REQ_LEN: usize = 16;
/// surface handle (low, high), shape, reserved 0.
pub const CURSOR_SHAPE_REQ_LEN: usize = 16;
pub const SCENE_REMOVE_REQ_LEN: usize = 8;
/// handle, x, y, width, height, z, window id, replaced handle, flags, reserved.
pub const SCENE_SUBMIT_LAYER_REQ_LEN: usize = 48;
/// handle, z, reserved.
pub const SCENE_REMOVE_LAYER_REQ_LEN: usize = 16;
/// owner pid, window id.
pub const FOCUS_SET_WINDOW_REQ_LEN: usize = 8;
pub const INDICATOR_REQ_LEN: usize = 8;
pub const OUTPUT_LAYOUT_REQ_LEN: usize = 4;
pub const SCREENSHOT_REQ_LEN: usize = 8;
pub const OUTPUT_SCALE_REQ_LEN: usize = 8;
pub const NIGHT_LIGHT_REQ_LEN: usize = 4;
pub const LID_REQ_LEN: usize = 4;
pub const FRAME_TRACE_REQ_LEN: usize = 4;
pub const IDLE_BLANK_REQ_LEN: usize = 4;
pub const PRESENT_REQ_LEN: usize = 32;
/// PRESENT's body, then the handle the layer shows now.
pub const PRESENT_LAYER_REQ_LEN: usize = 40;
/// The layer's handle and the content's, its corner, its flags, the box of
/// it that changed, the box of its surface drawn and a zero word: sixteen
/// words.
pub const LAYER_CONTENT_REQ_LEN: usize = 64;
/// The most boxes a parts block tells apart.
pub const MAX_PARTS: usize = 4;
/// A parts block: a count of 1 to MAX_PARTS, then MAX_PARTS boxes of four
/// words, those past the count zero.
pub const PARTS_LEN: usize = 4 + MAX_PARTS * 16;
/// PRESENT_LAYER's body, then a parts block.
pub const PRESENT_LAYER_PARTS_REQ_LEN: usize = PRESENT_LAYER_REQ_LEN + PARTS_LEN;
/// LAYER_CONTENT's body, then a parts block.
pub const LAYER_CONTENT_PARTS_REQ_LEN: usize = LAYER_CONTENT_REQ_LEN + PARTS_LEN;
pub const BRIGHTNESS_STEP_REQ_LEN: usize = 4;
pub const SET_VCP_REQ_LEN: usize = 12;
pub const ACCESSIBILITY_GET_REQ_LEN: usize = 4;
pub const DISPLAY_INFO_DATA_LEN: usize = 16;
pub const DISPLAY_INFO_RESP_LEN: usize = STATUS_LEN + DISPLAY_INFO_DATA_LEN;
