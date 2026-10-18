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

//! What a client can learn from the compositor: beyond a status, only
//! DISPLAY_INFO's canvas geometry (16 bytes), DISPLAY_HINT's flags and
//! OUTPUT_LAYOUT's mode and OUTPUT_SCALE's scale (4 bytes each), an
//! output's accessibility settings and the arrangement's counts and scales
//! (16 and 32 bytes of numbers), and SCREENSHOT's size; OUTPUT_NOTICE
//! answers nothing at all (`notice_tests`); no op hands
//! back pixels, a surface or a handle, and the screen's pixels reach only
//! the surface a screen capture holder shared (`screenshot_tests`).

use crate::protocol::*;

#[test]
fn no_op_answers_with_pixels_or_handles() {
    let ops = [
        OP_HEALTHCHECK,
        OP_SCENE_SUBMIT,
        OP_DAMAGE_COMMIT,
        OP_FOCUS_SET,
        OP_INPUT_SUBSCRIBE,
        OP_CURSOR_UPDATE,
        OP_SCENE_REMOVE,
        OP_DISPLAY_INFO,
        OP_DISPLAY_HINT,
        OP_INDICATOR,
        OP_OUTPUT_LAYOUT,
        OP_SCREENSHOT,
        OP_OUTPUT_SCALE,
        OP_NIGHT_LIGHT,
        OP_ACCESSIBILITY,
        OP_ACCESSIBILITY_GET,
        OP_OUTPUT_ARRANGEMENT,
        OP_OUTPUT_NOTICE,
    ];
    assert_eq!(ops.to_vec(), (1u16..=18).collect::<Vec<_>>(), "every op the dispatch knows");
    assert_eq!(DISPLAY_INFO_RESP_LEN, STATUS_LEN + 16, "the canvas's geometry and nothing else");
    assert_eq!((INDICATOR_REQ_LEN, OUTPUT_LAYOUT_REQ_LEN), (8, 4));
}
