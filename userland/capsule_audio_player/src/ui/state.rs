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

//! View state for the shell: which screen is up, where each list is scrolled,
//! and the live search buffer fed from the keyboard path.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::library::Library;

#[derive(Clone, Copy, PartialEq)]
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

pub struct UiState {
    pub view: View,
    pub scroll: usize,
    pub query: String,
    pub lib_tab: usize,
    pub hover: Option<usize>,
}

impl UiState {
    pub fn new() -> UiState {
        UiState { view: View::Home, scroll: 0, query: String::new(), lib_tab: 0, hover: None }
    }

    pub fn go(&mut self, v: View) {
        self.view = v;
        self.scroll = 0;
    }

    pub fn scroll_by(&mut self, delta: i32, len: usize, visible: usize) {
        let max = len.saturating_sub(visible);
        let next = self.scroll as i32 + delta;
        self.scroll = next.clamp(0, max as i32) as usize;
    }
}

fn folded(hay: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let hb = hay.as_bytes();
    let nb = needle.as_bytes();
    if nb.len() > hb.len() {
        return false;
    }
    (0..=hb.len() - nb.len()).any(|i| (0..nb.len()).all(|j| hb[i + j].eq_ignore_ascii_case(&nb[j])))
}

pub fn matches(lib: &Library, i: usize, q: &str) -> bool {
    match lib.get(i) {
        Some(t) => folded(&t.title, q) || folded(&t.artist, q) || folded(&t.format, q),
        None => false,
    }
}

pub fn filtered(lib: &Library, q: &str) -> Vec<usize> {
    (0..lib.tracks.len()).filter(|&i| matches(lib, i, q)).collect()
}
