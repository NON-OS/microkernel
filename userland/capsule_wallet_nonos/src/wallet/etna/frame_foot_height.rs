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

//! How tall the frame's foot is: the pinned actions, the notice row and the
//! status line.

use super::super::frame_spec::FrameSpec;
use super::super::roles::Role;
use super::super::tokens::{ROOM, SIDE, STATUS_H, TALL, TIGHT};

fn buttons_height(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        ROOM * 2 + TALL * n + TIGHT * (n - 1) + 1
    }
}

/// The notice row's height: the wallet's last word, wrapped to the column,
/// with room above it. None without a status line or a word to say.
pub(super) fn notice_height(spec: &FrameSpec) -> u32 {
    let text = super::super::notice::now();
    if spec.status.is_empty() || text.is_empty() {
        return 0;
    }
    let w = super::super::current::now().column.saturating_sub(2 * SIDE) as i32;
    super::super::wrap::wrapped_height(w, Role::Status, text) as u32 + TIGHT
}

pub fn foot_height(spec: &FrameSpec) -> u32 {
    let status = if spec.status.is_empty() { 0 } else { STATUS_H };
    buttons_height(spec.footer.len() as u32) + notice_height(spec) + status
}
