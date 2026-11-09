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

use nonos_app_skeleton::{measure_ttf, PaintBuffer};
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::paint::mixer::lerp_argb;
use nonos_toolkit::widgets::{Card, Controls, IconButton, Kind, State};

use crate::browser::paint::clip;
use crate::browser::paint::pages::cells::{area, below, px};
use crate::browser::paint::pages::tab_chip;
use crate::browser::paint::pages::tab_head::{head, HEAD_H, PAD};
use crate::browser::paint::start::tile_color;
use crate::browser::paint::tokens::{category, palette};
use crate::browser::shell::cards::TabCard;
use crate::browser::shell::Shell;
use crate::browser::ui::fit;
use crate::browser::ui::grid_geom::close_rect;
use crate::browser::ui::list_geom::ListGeom;
use crate::browser::ui::page_part::PagePart;
use crate::browser::ui::rect::Rect;

const HOST_PX: f32 = 26.0;
const BODY_RADIUS: u32 = 10;
const TINT: u32 = 40;

/// One card per tab in `shown`, switching to it, with its close button and
/// its group chip, which moves it to the next group, laid over the card;
/// the active one is selected. Each takes clicks only below the band and
/// is lit only where a click on it lands.
pub fn paint(
    shell: &Shell,
    shown: &[usize],
    g: &ListGeom,
    fb: &mut PaintBuffer,
    c: &mut Controls<PagePart>,
) {
    let cols = g.cols();
    for (j, &i) in shown.iter().enumerate() {
        let r = g.grid_rect(j, cols);
        let Some(t) = shell.cards.get(i) else { continue };
        if r.bottom() <= g.band || r.y >= fb.height as i32 {
            continue;
        }
        let (label, k) = tab_chip::look(shell, t.group);
        let (x, chip) = (close_rect(r), tab_chip::pill(&label, k));
        let at = tab_chip::rect(r, &chip);
        let (id, tab) = (u64::from(t.serial), PagePart::Tab(i, u64::from(t.serial)));
        let parts = [(r, tab), (x, PagePart::Close(i, id)), (at, PagePart::Chip(i, id))];
        for (b, p) in parts {
            c.push(area(below(b, g.band)), p);
        }
        let st = State { selected: t.active, ..c.state(tab) };
        Card::new().state(st).draw(fb, area(r));
        card(fb, r, t);
        let st = c.state(PagePart::Close(i, id));
        IconButton::new(IconId::Close).kind(Kind::Plain).state(st).draw(fb, area(x));
        chip.state(c.state(PagePart::Chip(i, id))).draw(fb, area(at));
    }
}

/* The card's head and its body: the host large on the site's colour. */
fn card(fb: &mut PaintBuffer, r: Rect, t: &TabCard) {
    let k = tile_color(&t.host);
    head(fb, r, &t.title, &t.host, k);
    let (color, pad, big) = (category(k).0, px(PAD), scale().font(HOST_PX));
    let body = Rect::new(r.x + pad, r.y + px(HEAD_H), r.w - 2 * pad, r.h - px(HEAD_H) - pad);
    clip::round(fb, body, scale().px(BODY_RADIUS), lerp_argb(palette().surface, color, TINT));
    let host = fit::truncate_to(&t.host, body.w - 2 * pad, |s| measure_ttf(s, big));
    let bx = body.x + (body.w - measure_ttf(&host, big)) / 2;
    let by = body.y + (body.h - line_height(big)) / 2;
    fb.text_ttf(bx, by, &host, color, big);
}
