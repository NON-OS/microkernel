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

//! The top card on any other disk, as it always was: the disk, its size,
//! what it holds now and what erasing it destroys. A NØNOS whose data an
//! update cannot keep says why where "holds now" would say it holds NØNOS.
//! And, in the plan's place on an update row whose data is not ready, how
//! to get it ready.

use nonos_app_skeleton::PaintBuffer;
use nonos_blk_client::Disk;
use nonos_disk::Keep;

use super::confirm_warn::warn;
use crate::install::format::bytes;
use crate::install::state::State;
use crate::install::ui::keep_text::NOT_READY;
use crate::install::ui::metrics::Metrics;
use crate::install::ui::widgets::kv;
use crate::install::ui::wrap::{paragraph, Ink};
use crate::install::ui::{text, theme};

/// The card's four lines from `y`, inside the card's `x` and `w`.
pub fn disk_rows(
    state: &State,
    d: &Disk,
    fb: &mut PaintBuffer,
    m: &Metrics,
    x: u32,
    y: u32,
    w: u32,
) {
    let model = d.identity.map(|i| alloc::string::String::from(i.model_str()));
    let mut r = kv(fb, m, x, y, w, "disk", model.as_deref().unwrap_or(d.label()), false);
    let size = alloc::format!("{}, {}", bytes(d.bytes()), d.label());
    r = kv(fb, m, x, r, w, "size", &size, false);
    let held = match state.keep {
        Some(Keep::No(why)) => why.text(),
        _ => d.contents.text(),
    };
    r = kv(fb, m, x, r, w, "holds now", held, false);
    warn(fb, m, x, r, d.contents);
}

/// How to get the data ready to keep, and the keys that lead elsewhere.
pub fn not_ready(fb: &mut PaintBuffer, m: &Metrics, x: u32, y: u32, w: u32) {
    let y = paragraph(fb, x, y, w, NOT_READY, Ink::body(m, theme::FOREGROUND));
    let keys = "Press Down to erase and install instead, or Esc to choose another disk.";
    text::line(fb, x, y + m.unit, keys, theme::MUTED, m.body_px);
}
