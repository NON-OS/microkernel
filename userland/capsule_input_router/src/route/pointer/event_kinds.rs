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

//! Which pointer events move the cursor, and which are hit-tested.

use nonos_libc::{
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL,
    INPUT_KIND_TOUCH, INPUT_KIND_WHEEL,
};

pub(super) fn is_motion(kind: u16) -> bool {
    kind == INPUT_KIND_POINTER_REL || kind == INPUT_KIND_POINTER_ABS
}

pub(super) fn needs_hit_test(kind: u16) -> bool {
    matches!(
        kind,
        INPUT_KIND_BUTTON_DOWN | INPUT_KIND_BUTTON_UP | INPUT_KIND_TOUCH | INPUT_KIND_WHEEL
    )
}
