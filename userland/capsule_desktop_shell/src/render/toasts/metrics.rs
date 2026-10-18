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

//! The panel's measures, in logical pixels scaled to the screen (`px`), so
//! the toasts keep their size on every display's density.

use crate::render::ui_font::px;

const TOAST_MAX_WIDTH_LOGICAL: u32 = 460;
const TOAST_MIN_WIDTH_LOGICAL: u32 = 260;
const TOAST_RIGHT_INSET_LOGICAL: u32 = 12;
const TOAST_DOCK_GAP_LOGICAL: u32 = 10;

pub(super) fn toast_max_width() -> u32 {
    px(TOAST_MAX_WIDTH_LOGICAL)
}

pub(super) fn toast_min_width() -> u32 {
    px(TOAST_MIN_WIDTH_LOGICAL)
}

pub(super) fn toast_right_inset() -> u32 {
    px(TOAST_RIGHT_INSET_LOGICAL)
}

pub(super) fn toast_dock_gap() -> u32 {
    px(TOAST_DOCK_GAP_LOGICAL)
}
