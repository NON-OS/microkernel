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

//! Eight colours for telling things apart: tab groups, tags, labels.

use super::category_ink::flat;
use super::colour::Opaque;
use super::contrast::{luminance, LARGE};
use super::lift::lift;
use super::roles::{roles, Roles};

/// How many categories there are. An index wraps at this.
pub const CATEGORIES: usize = 8;

/*
 * Blue, cyan, purple, pink, orange, green, amber and a warm grey, spread round
 * the wheel so neighbours differ in hue, not only in lightness. Seeds, not what
 * is drawn: each is lifted the way a status colour is, brighter on a dark scheme
 * and darker on a light one until it clears 3:1, so only the lightness moves.
 */
const SEEDS: [u32; CATEGORIES] = [
    0xFF25_63EB,
    0xFF08_91B2,
    0xFF93_33EA,
    0xFFDB_2777,
    0xFFEA_580C,
    0xFF16_A34A,
    0xFFCA_8A04,
    0xFF78_716C,
];

/// One of the eight colours for telling things apart, saying nothing else.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Category(u8);

impl Category {
    /// The category at `i`, wrapped, so any stored number names one.
    pub const fn new(i: usize) -> Self {
        Self((i % CATEGORIES) as u8)
    }

    /// Where this category sits among the eight.
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    /// Its colour in the theme in force, reading at 3:1 on every flat ground.
    pub fn colour(self) -> Opaque {
        self.colour_in(&roles())
    }

    /// Its colour under the roles `r`.
    pub fn colour_in(self, r: &Roles) -> Opaque {
        self.colour_over(&flat(r), r.text.argb())
    }

    /// Its colour at 3:1, the non-text floor, on each of `grounds` under `text`.
    pub fn colour_over(self, grounds: &[u32], text: u32) -> Opaque {
        let light = grounds.iter().all(|&g| luminance(g) > luminance(text));
        Opaque::new(lift(SEEDS[self.index()], light, grounds, LARGE))
    }
}
