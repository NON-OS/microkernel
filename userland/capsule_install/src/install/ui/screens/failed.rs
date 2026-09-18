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

//! Why it stopped, in the words the writer gave, and what state the disk is
//! in. A write that stopped before the table left a disk with no partition
//! table, which firmware reads as empty; a read-back that failed left a
//! complete disk whose contents cannot be trusted. Both are said.

use nonos_app_skeleton::PaintBuffer;

use crate::install::format::bytes;
use crate::install::state::State;
use crate::install::ui::frame::Body;
use crate::install::ui::metrics::{BODY_PX, LINE_H};
use crate::install::ui::wrap::paragraph;
use crate::install::ui::{text, theme};

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let why = state
        .outcome
        .as_ref()
        .and_then(|o| o.error.as_deref())
        .or(state.notice.as_deref())
        .unwrap_or("stopped for a reason the writer did not name");
    text::line(fb, b.x, b.y, "The install did not complete.", theme::DANGER, BODY_PX);
    let mut y = paragraph(fb, b.x, b.y + LINE_H + 8, b.w, why, theme::FOREGROUND, BODY_PX, LINE_H);
    y += 12;

    if let Some(o) = state.outcome.as_ref() {
        // A receipt exists only once the table and the flush went down.
        let table_written = o.disk_guid[0] != b'-';
        let state_line = if table_written {
            "The disk has a complete table but its contents did not read back as written. Do not boot it."
        } else {
            "The disk has no partition table. Firmware will treat it as empty."
        };
        y = paragraph(fb, b.x, y, b.w, state_line, theme::MUTED, BODY_PX, LINE_H);
        let progress = alloc::format!("{} written before it stopped", bytes(o.bytes_written));
        text::line(fb, b.x, y + 8, &progress, theme::MUTED, BODY_PX);
    }
}
