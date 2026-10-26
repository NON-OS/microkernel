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
    INDICATOR_REQ_LEN, IPC_PAYLOAD_MAX, LID_REQ_LEN, NIGHT_LIGHT_REQ_LEN, OUTPUT_LAYOUT_REQ_LEN,
    OUTPUT_SCALE_REQ_LEN, SCENE_REMOVE_REQ_LEN, SCENE_SUBMIT_REQ_LEN, SCREENSHOT_REQ_LEN,
    SET_VCP_REQ_LEN, STATUS_LEN,
};
pub use ops::{
    OP_ACCESSIBILITY, OP_ACCESSIBILITY_GET, OP_BRIGHTNESS_GET, OP_BRIGHTNESS_STEP, OP_CURSOR_SHAPE,
    OP_CURSOR_UPDATE, OP_DAMAGE_COMMIT, OP_DISPLAY_HINT, OP_DISPLAY_INFO, OP_FOCUS_SET,
    OP_HEALTHCHECK, OP_INDICATOR, OP_INPUT_SOURCES, OP_INPUT_SUBSCRIBE, OP_LID, OP_NIGHT_LIGHT,
    OP_OUTPUT_ARRANGEMENT, OP_OUTPUT_LAYOUT, OP_OUTPUT_NOTICE, OP_OUTPUT_SCALE, OP_SCENE_REMOVE,
    OP_SCENE_SUBMIT, OP_SCREENSHOT, OP_SET_VCP,
};
