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

//! Downloads: every MP3 link the person asked for, newest last, each with
//! how far it has got and what can be done with it: cancel one running or
//! waiting, resume one the network stopped, play one that is in, remove a
//! finished one. With none, the page says how to add music.

extern crate alloc;

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::derive::with_alpha;
use nonos_toolkit::theme::{roles, Translucent};
use nonos_toolkit::widgets::{draw, text, Card, Kind, Progress, SectionHeader, State as Look};

use crate::fetch::list::{Act, Row, State};
use crate::fetch::row_text::{label, permille, status};
use crate::ui::drawn::{glow, labelled};
use crate::ui::fmt::count;
use crate::ui::geometry::{page_head, Rect};
use crate::ui::hover::Lit;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{line_h, BODY, ITEM, S3, S4, S6, SECONDARY};
use crate::ui::target::Target;

use super::downloads_geom::{buttons, clear_rect, empty_rect, row_rect, visible, CLEAR};

/// Whether the page offers Clear finished.
pub fn any_finished(rows: &[Row]) -> bool {
    rows.iter().any(|r| !matches!(r.state, State::Queued | State::Running | State::Stopping))
}

pub fn paint(fb: &mut PaintBuffer, icons: &Icons, r: Rect, rows: &[Row], scroll: usize, lit: &Lit) {
    let running = rows.iter().filter(|r| matches!(r.state, State::Running | State::Queued)).count();
    let sub = if rows.is_empty() {
        alloc::string::String::from("MP3 links you paste into Search download here, over the network you chose.")
    } else if running > 0 {
        count(running, "download in progress", "downloads in progress")
    } else {
        alloc::string::String::from("All done. Finished downloads are in /home/nonos/music.")
    };
    SectionHeader::new("Downloads").note(&sub).draw(fb, page_head(r).tk());
    if any_finished(rows) {
        let look = lit.of(Target::ClearFinished);
        labelled(fb, icons, clear_rect(r), CLEAR, None, Kind::Secondary, look);
    }
    if rows.is_empty() {
        empty(fb, icons, r);
        return;
    }
    for slot in 0..visible(r) {
        let Some(row) = rows.get(scroll + slot) else { break };
        one(fb, icons, row_rect(r, slot), row, |k| lit.of(Target::Fetch(scroll + slot, k)));
    }
}

/// The badge `b`, its corners `rad`, holding `glyph` at `size` on a wash of
/// `tint`.
fn badge(
    fb: &mut PaintBuffer,
    icons: &Icons,
    b: Rect,
    (glyph, size, rad): (Glyph, i32, u32),
    tint: u32,
) {
    draw::round(fb, b.tk(), rad, Translucent::new(with_alpha(tint, 0x22)));
    icons.centred(fb, b, size, glyph, tint);
}

fn one(fb: &mut PaintBuffer, icons: &Icons, rr: Rect, row: &Row, look: impl Fn(usize) -> Look) {
    let t = roles();
    Card::new().draw(fb, rr.tk());
    let (glyph, tint) = match row.state {
        State::Done(_) => (Glyph::Check, t.ok),
        State::Failed { resumable: false, .. } => (Glyph::Close, t.danger),
        _ => (Glyph::Download, t.accent),
    };
    let b = Rect::new(rr.x + S4, rr.cy() - 22, 44, 44);
    badge(fb, icons, b, (glyph, 22, 12), tint.argb());

    let labels: Vec<&str> = row.acts().iter().map(|&a| label(a)).collect();
    let left_of_buttons = buttons(rr, &labels).last().map_or(rr.right() - S4, |(_, b)| b.x - S4);
    let tx = b.right() + S4;
    let tw = (left_of_buttons - tx).max(0) as u32;
    let top = rr.y + S4;
    text::left(fb, tx as u32, top, &text::truncate(&row.name, tw, ITEM), t.text, ITEM);
    let said = status(row);
    let ink = match row.state {
        State::Failed { .. } => t.danger,
        State::Done(_) => t.ok,
        _ => t.text_2,
    };
    let line = text::truncate(&said, tw, SECONDARY);
    text::left(fb, tx as u32, top + line_h(ITEM) + 2, &line, ink, SECONDARY);
    if matches!(row.state, State::Running | State::Stopping) {
        let bar = Rect::new(tx, rr.bottom() - S4 - 6, tw as i32, 8).tk();
        Progress::new(permille(row).unwrap_or(0), 1000).draw(fb, bar);
    }
    for (k, b) in buttons(rr, &labels) {
        let lead = matches!(row.acts()[k], Act::Play | Act::Resume);
        if lead {
            glow(fb, b);
        }
        let kind = if lead { Kind::Primary } else { Kind::Secondary };
        labelled(fb, icons, b, labels[k], None, kind, look(k));
    }
}

/// No downloads yet: how music gets here.
fn empty(fb: &mut PaintBuffer, icons: &Icons, r: Rect) {
    let (body, t) = (empty_rect(r), roles());
    Card::new().draw(fb, body.tk());
    let b = Rect::new(body.x + S6, body.y + S6, 56, 56);
    badge(fb, icons, b, (Glyph::Download, 28, 14), t.accent.argb());
    let tx = b.right() + S6;
    let tw = (body.right() - S6 - tx).max(0) as u32;
    let mut y = body.y + S6;
    text::left(fb, tx as u32, y, "Bring your own music", t.text, ITEM);
    y += line_h(ITEM) + S3;
    let lines = [
        "1.  Copy the link to an MP3 file.",
        "2.  Press / or click Search, then Ctrl+V and Enter.",
        "3.  It downloads over Anyone or Nym into /home/nonos/music and plays.",
        "Files you copy into /home/nonos/music appear in the Library too.",
    ];
    for l in lines {
        let line = text::truncate(l, tw, BODY);
        let top = text::centre_top(y.max(0) as u32, line_h(BODY) as u32, BODY);
        text::left(fb, tx as u32, top, &line, t.text_2, BODY);
        y += line_h(BODY) + S3 / 2;
    }
}
