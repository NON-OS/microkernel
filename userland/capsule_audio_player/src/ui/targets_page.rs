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

//! The page's half of the target table: what the screen on show offers, in
//! the order a click has always tried them.

extern crate alloc;

use alloc::vec::Vec;

use super::geometry::Rect;
use super::screen::home_geom::{action_rect, card_rect, cols, see_all_rect, shelf_filled};
use super::screen::{downloads_geom as dl, home_geom, lib_geom, search_geom as sg, settings};
use super::shown::Shown;
use super::target::Target::{self, *};
use super::view::View;

/// The targets of the page `p`, after the chrome's.
pub fn page_targets(out: &mut Vec<(Target, Rect)>, s: &Shown, p: Rect) {
    match s.view {
        View::Home => home(out, p, s.tracks),
        View::Library => out.extend(lib_geom::rows(lib_geom::list_rect(p), s.scroll, s.rows)),
        View::Search => {
            out.extend(s.query.then(|| (PageClear, sg::clear_rect(p))));
            out.extend((s.address || s.rows > 0).then(|| (PagePlay, sg::play_rect(p))));
            if !s.address {
                out.extend(lib_geom::rows(sg::list_rect(p), s.scroll, s.rows));
            }
        }
        View::Settings => {
            out.push((Volume, settings::volume_rect(p)));
            out.extend((0..3).map(|i| (Toggle(i), settings::toggle_row(p, i))));
        }
        View::Downloads => {
            out.extend(s.finished.then(|| (ClearFinished, dl::clear_rect(p))));
            let shown = s.acts.iter().enumerate().skip(s.scroll).take(dl::visible(p));
            for (slot, (n, labels)) in shown.enumerate() {
                let row = dl::buttons(dl::row_rect(p, slot), labels);
                out.extend(row.map(|(k, b)| (Fetch(n, k), b)));
            }
        }
        View::NowPlaying => {}
    }
}

fn home(out: &mut Vec<(Target, Rect)>, p: Rect, n: usize) {
    let banner = Rect::new(p.x, p.y, p.w, home_geom::hero_h());
    out.extend((0..2).map(|i| (Hero(i), action_rect(banner, i))));
    let (k, shelves) = (cols(p) as usize, home_geom::shelves(p));
    let filled = (0..shelves).filter(|&s| shelf_filled(p, n, s));
    out.extend(filled.map(|s| (SeeAll(s), see_all_rect(p, s))));
    out.extend((0..n.min(k * shelves)).map(|i| (Tile(i), card_rect(p, i / k, (i % k) as i32))));
}
