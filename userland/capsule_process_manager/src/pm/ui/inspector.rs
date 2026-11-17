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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;

use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::clip;
use nonos_toolkit::widgets::{Align, Ink, Label, Rect};

use crate::pm::format::{state_label, u32_decimal};
use crate::pm::state::{Row, State};

use super::insp_geom::{content_w, content_x, pane_x};
use super::metrics::metrics;
use super::tint::state_tint;
use super::{insp_actions, insp_chips, insp_fields, insp_spark, text};

// The docked detail pane: identity, the numbers behind the selected row, its
// trend, its full grant list, and the action. It owns the whole window
// height, so it paints its own ground instead of sitting inside a content rect.
pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let (r, x, m) = (roles(), pane_x(fb.width), metrics());
    fb.fill_rect(x, 0, m.inspector_w, fb.height, r.sunken.argb());
    fb.fill_rect(x, 0, scale().hair(), fb.height, r.border.argb());
    let Some(row) = state.selected_row() else {
        return empty(fb, x);
    };
    let (left, w) = (content_x(fb.width), content_w());
    let (_, actions, _, _) = super::insp_geom::btn(fb.width, fb.height);
    let (gap, field_h) = (m.insp_section_gap, m.insp_field_h);
    let limit = actions.saturating_sub(gap);
    let mut y = heading(fb, left, m.pane_pad_top, w, row);
    /*
     * A short window has no room for every field above the action. The
     * fields used to run on through the buttons and the grant chips, so the
     * lines wrote over each other. They are clipped to the whole lines that
     * fit, and what follows them is drawn only where it fits too.
     */
    let lines = limit.saturating_sub(y) / field_h * field_h;
    let mut clip = fb.sub(0, 0, fb.width, y + lines);
    y = insp_fields::block(&mut clip, left, y, row, state.sys.mem_total_kb) + gap;
    let armed = state.pending_pid != 0 && state.pending_pid == row.pid;
    let protected = state.is_protected(row);
    if y + insp_spark::height() > limit {
        return insp_actions::paint(fb, state, protected, armed);
    }
    y = insp_spark::paint(fb, left, y, w, state.history.get(row.pid));
    if y + field_h <= limit {
        let mut buf = [0u8; 12];
        let n = u32_decimal(row.caps.count_ones(), &mut buf);
        y = insp_fields::field(fb, left, y, b"Authority", &buf[..n], Ink::Text);
        insp_chips::paint(fb, left, y, w, limit, row.caps);
    }
    insp_actions::paint(fb, state, protected, armed);
}

fn heading(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, row: &Row) -> u32 {
    let (r, m) = (roles(), metrics());
    let cut = clip(text::valid(row.name()), w, m.title_px).as_bytes();
    text::left(fb, x, y, cut, r.text, m.title_px);
    let next = y + line_height(m.title_px).max(1) as u32;
    let tint = state_tint(row.state).colour(&r);
    text::left(fb, x, next, state_label(row.state), tint, m.body_px);
    next + line_height(m.body_px).max(1) as u32 + m.insp_section_gap
}

fn empty(fb: &mut PaintBuffer, x: u32) {
    let at = Rect { x, y: 0, w: metrics().inspector_w, h: fb.height };
    Label::new("No process selected").ink(Ink::Muted).align(Align::Centre).draw(fb, at);
}
