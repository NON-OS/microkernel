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

//! Home: the first track in the library as the banner, then the library in
//! six-up shelves, in the order it was read. `home_geom.rs` is the one
//! geometry source, so the hit-test lands on exactly the tile or link the
//! painter drew.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Button, Kind, SectionHeader};

use crate::library::Library;
use crate::ui::geometry::Rect;
use crate::ui::hover::Lit;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::target::Target;
use crate::ui::widget::hero;

use super::home_geom::{card_rect, cols, see_all_rect, shelf_filled, shelf_head, shelves};
use super::home_geom::{hero_h, SEE_ALL};
use super::home_tile::tile;

// Plain names for what each shelf holds: the library, split in reading order.
// The player keeps no history or recommendations to sort by.
const SHELF_TITLES: [&str; 3] = ["Your library", "More in your library", "Also in your library"];

pub fn paint(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    lib: &Library,
    playing: Option<usize>,
    lit: &Lit,
) {
    let banner = Rect::new(r.x, r.y, r.w, hero_h());
    let feature = lib.tracks.first();
    let (eyebrow, title, copy, format, actions) = match feature {
        Some(t) => (
            "First in your library",
            t.title.as_str(),
            "Play starts it, Shuffle mixes the queue.",
            t.format.as_str(),
            [("Play", Glyph::Play), ("Shuffle", Glyph::Shuffle)],
        ),
        // Nothing ships with the system: the banner says how music gets here
        // and its buttons go there, not a Play with nothing to play.
        None => (
            "Welcome",
            "Your music lives here",
            "Paste an MP3 link into Search, or copy files into /home/nonos/music.",
            "",
            [("Paste a link", Glyph::Search), ("Downloads", Glyph::Download)],
        ),
    };
    let looks = [lit.of(Target::Hero(0)), lit.of(Target::Hero(1))];
    hero(fb, icons, banner, eyebrow, title, copy, ("NONOS AUDIO", format), actions, looks);

    let n = lib.tracks.len();
    for shelf in 0..shelves(r) {
        if !shelf_filled(r, n, shelf) {
            break;
        }
        SectionHeader::new(SHELF_TITLES[shelf.min(2)]).draw(fb, shelf_head(r, shelf).tk());
        let see_all = Button::new(SEE_ALL).kind(Kind::Plain).state(lit.of(Target::SeeAll(shelf)));
        see_all.draw(fb, see_all_rect(r, shelf).tk());
        for col in 0..cols(r) {
            let i = shelf * cols(r) as usize + col as usize;
            if i >= n {
                break;
            }
            let look = lit.of(Target::Tile(i));
            tile(fb, icons, card_rect(r, shelf, col), &lib.tracks[i], playing == Some(i), look);
        }
    }
}
