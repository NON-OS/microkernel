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
use nonos_toolkit::icons::draw;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Button, Card, Kind, Rect};

use crate::snake::input::target::Target;
use crate::snake::state::Game;
use crate::snake::ui::icon_table;
use crate::snake::ui::metrics::metrics;
use crate::snake::ui::rect::at;
use crate::snake::ui::setup_geom::{head, head_h, panel, start, SECTIONS};
use crate::snake::ui::setup_geom_rows::HEADS;
use crate::snake::ui::text::as_str;

use super::{setup_chips, setup_toggles};

const START: &str = "Start run";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    Card::new().draw(fb, backing(panel(w, h)));
    for section in 0..SECTIONS {
        band_head(fb, head(w, h, section), section);
    }
    setup_chips::paint(game, fb);
    setup_toggles::paint(game, fb);
    let look = game.hover.look(game.screen, Target::Start);
    Button::new(START).kind(Kind::Primary).state(look).draw(fb, start(w, h));
}

// The panel geometry is the content column; the ground behind it is a painter
// decoration and is grown from the same rect rather than measured again.
fn backing(r: Rect) -> Rect {
    let pad = metrics().pad;
    at(r.x.saturating_sub(pad), r.y.saturating_sub(pad), r.w + pad * 2, r.h + pad * 2)
}

fn band_head(fb: &mut PaintBuffer, r: Rect, section: usize) {
    let (theme, m) = (roles(), metrics());
    let mark = icon_table::option(section);
    let y = r.y + head_h().saturating_sub(m.icon_md) / 2;
    draw(fb, mark, r.x, y, m.icon_md, theme.muted.argb());
    let x = r.x + m.icon_md + m.gap_tight;
    let room = r.w.saturating_sub(m.icon_md + m.gap_tight);
    let cut = text::truncate(as_str(HEADS[section]), room, m.px_head);
    text::left(fb, x, r.y as i32, &cut, theme.text, m.px_head);
}
