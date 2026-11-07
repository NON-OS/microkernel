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

mod decode;
mod encode;
mod errno;
mod header;
mod limits;
mod ops;

pub use decode::parse;
pub use encode::{response_header, write_status};
pub use errno::{
    E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_VERSION, E_BUSY, E_INVAL, E_IO, E_NOENT, E_PERM,
};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use limits::{
    ACCESSIBILITY_GET_REQ_LEN, BRIGHTNESS_STEP_REQ_LEN, CURSOR_SHAPE_REQ_LEN,
    CURSOR_UPDATE_REQ_LEN, DAMAGE_COMMIT_REQ_LEN, DISPLAY_INFO_RESP_LEN, FOCUS_SET_REQ_LEN,
    FOCUS_SET_WINDOW_REQ_LEN, FRAME_TRACE_REQ_LEN, IDLE_BLANK_REQ_LEN, INDICATOR_REQ_LEN,
    IPC_PAYLOAD_MAX, LAYER_CONTENT_PARTS_REQ_LEN, LAYER_CONTENT_REQ_LEN, LID_REQ_LEN, NIGHT_LIGHT_REQ_LEN, OUTPUT_LAYOUT_REQ_LEN, OUTPUT_SCALE_REQ_LEN,
    MAX_PARTS, PARTS_LEN, PRESENT_LAYER_PARTS_REQ_LEN, PRESENT_LAYER_REQ_LEN, PRESENT_REQ_LEN, SCENE_REMOVE_LAYER_REQ_LEN, SCENE_REMOVE_REQ_LEN,
    SCENE_SUBMIT_LAYER_REQ_LEN, SCENE_SUBMIT_REQ_LEN, SCREENSHOT_REQ_LEN, SET_VCP_REQ_LEN,
    STATUS_LEN,
};
pub use ops::{
    OP_ACCESSIBILITY, OP_ACCESSIBILITY_GET, OP_BRIGHTNESS_GET, OP_BRIGHTNESS_STEP, OP_CURSOR_SHAPE,
    OP_CURSOR_UPDATE, OP_DAMAGE_COMMIT, OP_DISPLAY_HINT, OP_DISPLAY_INFO, OP_FOCUS_SET,
    OP_FOCUS_SET_WINDOW, OP_FRAME_TIME, OP_FRAME_TRACE, OP_HEALTHCHECK, OP_IDLE_BLANK,
    OP_INDICATOR, OP_INPUT_SOURCES, OP_INPUT_SUBSCRIBE, OP_LAYER_CONTENT, OP_LAYER_CONTENT_PARTS, OP_LID, OP_NIGHT_LIGHT,
    OP_OUTPUT_ARRANGEMENT, OP_OUTPUT_LAYOUT, OP_OUTPUT_NOTICE, OP_OUTPUT_RECTS, OP_OUTPUT_SCALE,
    OP_PRESENT, OP_PRESENT_LAYER, OP_PRESENT_LAYER_PARTS, OP_SCANOUT_PEER, OP_SCENE_REMOVE, OP_SCENE_REMOVE_LAYER,
    OP_SCENE_SUBMIT, OP_SCENE_SUBMIT_LAYER, OP_SCREENSHOT, OP_SET_VCP,
};
