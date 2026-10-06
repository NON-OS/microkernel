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

use core::cell::Cell;

use super::ctx::Ctx;

/// Layout data a box carries beside its style.
#[derive(Default)]
pub(crate) struct BoxAux {
    /* Where normal flow would have put this box, with the layout state of
     * its container there, recorded as flow passes it by. An out-of-flow box
     * reads it for an axis its insets leave auto: the CSS static position. */
    pub(crate) flow_at: Cell<Option<(i32, i32, Ctx)>>,
    /* An image's natural size in px, once its header has been read. */
    pub(crate) natural: Option<(u32, u32)>,
    /* The width and height attributes of an <img>: presentational hints for
     * its size, and together its aspect ratio before the image arrives. */
    pub(crate) attr: [Option<u32>; 2],
    /* Min-content and max-content widths, measured once per layout pass. */
    pub(crate) intrinsic: super::super::contexts::intrinsic_memo::IntrinsicMemo,
}
