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

//! The receipt: what was written, what was read back, and the identifiers
//! a firmware boot menu will show for this disk. Then the one instruction
//! that matters: take the stick out before restarting.

use nonos_app_skeleton::PaintBuffer;

use crate::install::format::{bytes, hex_prefix};
use crate::install::state::State;
use crate::install::ui::frame::Body;
use crate::install::ui::metrics::{BODY_PX, LINE_H};
use crate::install::ui::widgets::{card, kv};
use crate::install::ui::{text, theme};

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let Some(o) = state.outcome.as_ref() else { return };
    text::line(
        fb,
        b.x,
        b.y,
        "NØNOS is on the disk, and every sector read back as written.",
        theme::OK,
        BODY_PX,
    );

    let inner = card(fb, b.x, b.y + LINE_H + 12, b.w, 6 * LINE_H + 40, "receipt");
    let (x, w) = (b.x + 16, b.w - 32);
    let mut r = kv(fb, x, inner, w, "written", &bytes(o.bytes_written), false);
    r = kv(fb, x, r, w, "read back", &bytes(o.bytes_verified), false);
    r = kv(fb, x, r, w, "write time", &alloc::format!("{} s", o.seconds), false);
    r = kv(fb, x, r, w, "kernel measurement", &hex_prefix(&state.boot.kernel_blake3), true);
    let disk = core::str::from_utf8(&o.disk_guid).unwrap_or("");
    let part = core::str::from_utf8(&o.partition_guid).unwrap_or("");
    r = kv(fb, x, r, w, "disk", disk, true);
    kv(fb, x, r, w, "partition", part, true);

    let y = b.y + 7 * LINE_H + 76;
    text::line(
        fb,
        b.x,
        y,
        "Remove the USB stick, then press Enter to restart.",
        theme::FOREGROUND,
        BODY_PX,
    );
    text::line(
        fb,
        b.x,
        y + LINE_H,
        "The computer boots NØNOS from its own disk from now on.",
        theme::MUTED,
        BODY_PX,
    );
}
