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
use nonos_toolkit::widgets::rect::inset;
use nonos_toolkit::widgets::{Ink, Pill, Row, Separator, State as Look, Tone};

use crate::pm::format::u32_decimal;
use crate::pm::security::{Alert, Level};
use crate::pm::state::State;

use super::super::chrome::Rect;
use super::super::hit::Target;
use super::super::hover_look::pointed;
use super::super::metrics::card_metrics;
use super::super::text;
use super::sec_geom::{self, ALERT_GAP, ALERT_H};

pub fn tint(level: Level) -> Tone {
    match level {
        Level::Info => Tone::Accent,
        Level::Warn => Tone::Warn,
        Level::Critical => Tone::Danger,
    }
}

// One finding as a shared row across the card: the process it is about, its pid
// at the right, the message under them, and the level as a pill in the row's
// trailing box. A system-wide finding carries pid 0, which prints as a dash
// rather than a zero nobody can look up. Selection reads the same here as in
// the table, and a soft rule sits in the gap above every finding but the first.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect, alert: &Alert, slot: usize) {
    let (index, y) = (state.alert_scroll + slot, r.y + sec_geom::row_y(slot));
    let (s, pad) = (scale(), card_metrics().panel_pad);
    let at = Rect { y, h: s.px(ALERT_H), ..*r };
    if slot > 0 {
        let gap = Rect { y: y - s.px(ALERT_GAP), h: s.px(ALERT_GAP), ..inset(*r, pad) };

        Separator::new().soft().draw(fb, gap);
    }
    let mut buf = [0u8; 12];
    let n = u32_decimal(alert.pid, &mut buf);
    let pid = if alert.pid == 0 { "-" } else { text::valid(&buf[..n]) };
    let pill = Pill::new(text::valid(alert.level.label())).tone(tint(alert.level));
    let look = pointed(state, |t| t == Target::Finding(index));
    let look = Look { selected: index == state.alert_sel, ..look };
    let row = Row::new(text::valid(alert.name())).value(pid).value_ink(Ink::Muted);
    let row = row.note(text::valid(alert.msg)).trailing(pill.size().0).state(look);
    row.draw(fb, at);
    pill.draw(fb, row.trailing_rect(at));
}
