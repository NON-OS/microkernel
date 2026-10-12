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

//! What a client can learn from the compositor: of its ops only
//! DISPLAY_INFO answers more than a status, and that is the canvas's
//! geometry (16 bytes); no op hands back pixels, a surface or a handle, so
//! no client reads another's screen through the compositor. INDICATOR's
//! request is exactly kind and on, eight bytes.

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
    ];
    assert_eq!(ops.to_vec(), (1u16..=10).collect::<Vec<_>>(), "every op the dispatch knows");
    assert_eq!(DISPLAY_INFO_RESP_LEN, STATUS_LEN + 16, "the canvas's geometry and nothing else");
    assert_eq!(INDICATOR_REQ_LEN, 8);
}
