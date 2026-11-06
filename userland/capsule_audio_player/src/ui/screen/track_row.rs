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

//! One row of the track table. The shared Row is the body, and draws hover,
//! press and the playing row; the player's columns sit on it.

extern crate alloc;

use alloc::{format, string::String};
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Row, State};

use crate::library::Track;
use crate::ui::art::cover;
use crate::ui::fmt::mmss;
use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{line_h, DATA, ITEM, R_THUMB, SECONDARY};
use crate::ui::widget::equalizer;

use super::track_cols::{at, cols};

/// Track `t` as row `n` of the list in `r`. `look.selected` is the playing
/// row, which shows the output's `level` in place of its number.
pub fn row(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    (t, n): (&Track, usize),
    queued: bool,
    look: State,
    level: u16,
) {
    let (c, th, playing) = (cols(r), roles(), look.selected);
    Row::new("").state(look).draw(fb, r.tk());
    if playing {
        equalizer(fb, c.index.centred(16, 16), level);
    } else {
        let (x, top) = at(c.index, DATA);
        text::left(fb, x, top, &format!("{n:02}"), th.muted, DATA);
    }
    cover(fb, c.art, &t.path, R_THUMB);
    let fg = if playing { th.accent } else { th.text };
    let (tx, w) = (c.title.tk().x, c.title.tk().w);
    let ty = r.cy() - (line_h(ITEM) + line_h(SECONDARY)) / 2;
    text::left(fb, tx, ty, &text::truncate(&t.title, w, ITEM), fg, ITEM);
    let artist = text::truncate(&t.artist, w, SECONDARY);
    text::left(fb, tx, ty + line_h(ITEM), &artist, th.text_2, SECONDARY);
    /* The album its tags name, or the file's format when they name none. */
    let name = if t.album.is_empty() { &t.format } else { &t.album };
    let album = text::truncate(name, c.format.tk().w, DATA);
    let (x, top) = at(c.format, DATA);
    text::left(fb, x, top, &album, th.text_2, DATA);
    /* A track not loaded yet has no known length: "--", not "0:00". */
    let time = if t.dur_ms == 0 { String::from("--") } else { mmss(t.dur_ms) };
    let (x, top) = at(c.time, DATA);
    text::right(fb, x + c.time.tk().w, top, &time, th.text_2, DATA);
    if queued {
        icons.centred(fb, c.pip, 18, Glyph::Check, th.ok.argb());
    }
}
