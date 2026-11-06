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

//! Which screen is up, the destinations the sidebar offers and the Library's
//! tabs. Pure, so the proofs can place the controls each screen shows.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum View {
    Home,
    Library,
    Search,
    NowPlaying,
    Downloads,
    Settings,
}

// Every destination here shows the person's own music, read from
// /home/nonos/music, or what they are downloading into it. The player has no
// radio, playlist store or catalogue behind it, so it offers none of those
// pages rather than painting made-up ones.
pub const NAV: [(View, &str); 5] = [
    (View::Home, "Home"),
    (View::Library, "Library"),
    (View::Search, "Search"),
    (View::NowPlaying, "Now Playing"),
    (View::Downloads, "Downloads"),
];

pub const LIB_TABS: [&str; 4] = ["Songs", "Artists", "Albums", "Queued"];
