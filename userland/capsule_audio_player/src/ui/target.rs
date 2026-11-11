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

//! What the pointer can be over. A slider answers a little past its groove,
//! by as much as its click always did.

use super::geometry::Rect;
use super::metrics::px;
use super::view::View;

/// A transport control, in the bar across the foot or in the right rail.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Key {
    Play,
    Prev,
    Next,
    Shuffle,
    Repeat,
    Mute,
    Seek,
    Volume,
}

/// A control the window shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Target {
    Bar(Key),
    Rail(Key),
    /// The `n`th row of the rail's "Up next".
    Queue(usize),
    Nav(View),
    /// The top bar's search field, and its clear mark.
    Field,
    Clear,
    /// Home: a banner action, a shelf's "See all", the tile of track `n`.
    Hero(usize),
    SeeAll(usize),
    Tile(usize),
    /// Library: a tab. Library and Search: the `n`th row of the list shown.
    Tab(usize),
    Row(usize),
    /// Search: the page's Clear, and its Play now or Download.
    PageClear,
    PagePlay,
    /// Settings: a switch's row, and the volume.
    Toggle(usize),
    Volume,
    /// Downloads: Clear finished, and button `k` of download `n`.
    ClearFinished,
    Fetch(usize, usize),
}

/// Where `t`, drawn in `r`, answers the pointer.
pub fn reach(t: Target, r: Rect) -> Rect {
    match t {
        Target::Bar(Key::Seek | Key::Volume) => r.inset(-px(10)),
        Target::Rail(Key::Seek | Key::Volume) => r.inset(-px(8)),
        Target::Volume => r.inset(-px(12)),
        _ => r,
    }
}
