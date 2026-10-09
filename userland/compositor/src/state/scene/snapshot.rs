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

use super::layer::{Layer, MAX_LAYERS};
use super::table::SceneTable;

impl SceneTable {
    // Bottom-to-top draw order. Layers rank by z band first, then by raise
    // stamp, so inside a band the most recently raised window draws on top and
    // every other window keeps the place its own last raise gave it. The window
    // manager stacks windows the same way (each raise takes a new, higher z)
    // and tells the compositor about every raise, so the window drawn on top at
    // a point is the window its hit test hands a click at that point to.
    //
    // This used to lift only the focused window above its peers and leave every
    // other one in table order. With three windows the one drawn second from
    // the top was then not the one the window manager had second, and a click
    // on what was visibly on top went to a window underneath it.
    pub fn z_sorted_snapshot(&self) -> ([Layer; MAX_LAYERS], usize) {
        let mut out = [Layer::default(); MAX_LAYERS];
        let mut n = 0;
        for layer in self.entries.iter().filter(|l| l.in_use) {
            out[n] = *layer;
            n += 1;
        }
        // Composite rank: the band in the high half, the raise stamp in the low
        // half, so a raise reorders a layer among its peers and never moves it
        // out of its band (an application window never covers a layer of a
        // higher band, nor sinks under the desktop's).
        let rank = |l: &Layer| -> u64 { ((l.z as u64) << 32) | l.stack as u64 };
        let mut i = 1;
        while i < n {
            let mut j = i;
            while j > 0 && rank(&out[j - 1]) > rank(&out[j]) {
                out.swap(j - 1, j);
                j -= 1;
            }
            i += 1;
        }
        (out, n)
    }
}
