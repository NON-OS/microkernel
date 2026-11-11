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
use nonos_toolkit::widgets::Rect;

use crate::about::data::display::virtio_announced;
use crate::about::data::present;
use crate::about::state::State;

use super::super::card::{self, titled};
use super::super::kv::{kv, row_h};
use super::super::metrics::metrics;
use super::{chain, display_surface};

/// The gap between the chain and the backend row, in logical pixels.
const PATH_GAP: u32 = 14;

fn path_h() -> u32 {
    card::overhead() + metrics().chain_h + scale().px(PATH_GAP) + row_h()
}

pub fn content_h(_rect: &Rect) -> u32 {
    display_surface::height() + metrics().card_gap + path_h()
}

pub fn paint(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    let mut pane = fb.sub(rect.x, rect.y, rect.w, rect.h);
    let y = -(state.scroll as i32);
    display_surface::paint(&mut pane, y, rect.w);
    path(&mut pane, y + (display_surface::height() + metrics().card_gap) as i32, rect.w);
}

// The chain and the row under it are the same fact drawn twice: the hops the pixel
// takes, then the backend string those hops add up to. Which path it is comes
// from the same service lookup the compositor makes, asked as the card paints.
fn path(fb: &mut PaintBuffer, y: i32, w: u32) {
    let (m, top) = (metrics(), titled(fb, 0, y, w, path_h(), b"Present path"));
    let way = present::path(virtio_announced());
    chain::paint(fb, m.card_pad, top, &way.hops);
    let row_y = top + (m.chain_h + scale().px(PATH_GAP)) as i32;
    kv(fb, m.card_pad, row_y, card::inner(w), b"Backend", way.backend, false);
}
