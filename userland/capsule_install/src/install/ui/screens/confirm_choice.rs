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

//! The top card on a disk an update could keep: the disk, the row that
//! updates NØNOS and keeps the data and the row that erases it, the lit one
//! marked as a disk on the list is, and what the lit row does to the disk.
//! An update row whose data is not ready is drawn muted.

use nonos_app_skeleton::PaintBuffer;
use nonos_blk_client::Disk;

use super::confirm_warn::warn;
use crate::install::format::bytes;
use crate::install::state::{Choice, State};
use crate::install::ui::metrics::Metrics;
use crate::install::ui::text::top_of;
use crate::install::ui::widgets::kv;
use crate::install::ui::{text, theme};

const ROWS: [(&str, Choice); 2] = [
    ("Update NØNOS (keep my data)", Choice::Update),
    ("Erase everything and install", Choice::Erase),
];

const KEPT: &str = "your data volume and its key are kept; the system is rewritten";
const NOT_READY: &str = "not ready: prepare it on your installed NØNOS first";

/// The card's four lines from `y`, inside the card's `x` and `w`.
pub fn choices(state: &State, d: &Disk, fb: &mut PaintBuffer, m: &Metrics, x: u32, y: u32, w: u32) {
    let model = d.identity.map(|i| alloc::string::String::from(i.model_str()));
    let name = model.as_deref().filter(|s| !s.is_empty()).unwrap_or(d.label());
    let disk = alloc::format!("{name}, {}, {}", bytes(d.bytes()), d.label());
    let mut r = kv(fb, m, x, y, w, "disk", &disk, false);
    for (said, choice) in ROWS {
        let lit = state.choice == choice;
        if lit {
            let (bar, h) = (m.scale.px(3), m.line_h - m.unit);
            fb.fill_round(x - m.unit, r, w + 2 * m.unit, m.line_h, m.radius, theme::SELECTED_BG);
            fb.fill_rect(x - m.unit, r + m.unit / 2, bar, h, theme::ACCENT);
        }
        let colour = match (choice, lit) {
            (Choice::Update, _) if !state.keep_ready() => theme::MUTED,
            (_, true) => theme::TITLE,
            (_, false) => theme::FOREGROUND,
        };
        text::line(fb, x + m.unit, top_of(r, m.line_h, m.body_px), said, colour, m.body_px);
        r += m.line_h;
    }
    let (line, colour) = match (state.choice, state.keep_ready()) {
        (Choice::Erase, _) => return warn(fb, m, x, r, d.contents),
        (Choice::Update, true) => (KEPT, theme::OK),
        (Choice::Update, false) => (NOT_READY, theme::WARN),
    };
    text::line(fb, x, top_of(r, m.line_h, m.body_px), line, colour, m.body_px);
}
