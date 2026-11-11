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
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Card, Pill};

use crate::pm::state::State;

use super::super::chrome::Rect;
use super::super::metrics::{card_metrics, metrics};
use super::super::text;
use super::ovw_cards::sub_n;
use super::{sec_geom, sec_row};

pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let y = r.y + sec_geom::list_y();
    let (c, h) = (roles(), r.h.saturating_sub(sec_geom::list_y()));
    let (cm, px) = (card_metrics(), metrics().body_px);
    let (pad, head_h) = (cm.panel_pad, cm.panel_head_h);
    Card::new().draw(fb, Rect { y, h, ..*r });
    let top = text::centred_top(y, head_h, px);
    text::left(fb, r.x + pad, top, b"FINDINGS", c.text, px);
    headline(state, fb, r.x + r.w.saturating_sub(pad), y);
    if state.alerts.is_empty() {
        let clear = b"No findings: watched services up, nothing pinned at full cpu, admin held by init alone.";
        text::left(fb, r.x + pad, y + head_h, clear, c.muted, px);
        return;
    }
    for slot in 0..sec_geom::visible(r.h) {
        match state.alerts.get(state.alert_scroll + slot) {
            Some(alert) => sec_row::paint(state, fb, r, alert, slot),
            None => break,
        }
    }
}

// The worst level and how many findings sit at it, as one toolkit pill whose
// dot carries the level's tone, centred down the head band. Level derives Ord
// worst-last, so max() is the headline without a ranking table of its own.
fn headline(state: &State, fb: &mut PaintBuffer, right: u32, y: u32) {
    let level = match state.alerts.iter().map(|a| a.level).max() {
        Some(level) => level,
        None => return,
    };
    let (label, mut buf) = (level.label(), [0u8; 32]);
    let k = label.len().min(16);
    buf[..k].copy_from_slice(&label[..k]);
    let count = state.alerts.iter().filter(|a| a.level == level).count();
    let n = k + sub_n(&mut buf[k..], b" ", count as u32, b"");
    let pill = Pill::new(text::valid(&buf[..n])).tone(sec_row::tint(level)).dot();
    let w = pill.size().0;
    pill.draw(fb, Rect { x: right.saturating_sub(w), y, w, h: card_metrics().panel_head_h });
}
