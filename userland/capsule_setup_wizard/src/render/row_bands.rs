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

//! The rows of a setup frame that differ from the one on screen, as a few
//! bands of whole rows. Each screen is drawn whole off screen; only these
//! rows are copied to the surface the compositor reads and only these bands
//! are committed, so a key that moves a highlight or types a letter composes
//! the rows it changed, not the whole screen.

/// At most this many bands a frame; a row past the last band's reach joins
/// the last band.
pub const MAX_BANDS: usize = 4;
/// A changed row this close below a band joins it: two commits a few rows
/// apart cost more than composing the rows between them.
pub const JOIN_GAP: usize = 16;

/// `rows` rows from row `y`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Band {
    pub y: usize,
    pub rows: usize,
}

impl Band {
    fn end(&self) -> usize {
        self.y.saturating_add(self.rows)
    }
}

/// The bands of one frame, in row order, covering every row added.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bands {
    at: [Band; MAX_BANDS],
    n: usize,
}

impl Bands {
    pub const fn new() -> Self {
        Self { at: [Band { y: 0, rows: 0 }; MAX_BANDS], n: 0 }
    }

    /// One band of every row: a frame drawn on the surface itself.
    pub fn whole(rows: usize) -> Self {
        let mut b = Self::new();
        if rows > 0 {
            b.at[0] = Band { y: 0, rows };
            b.n = 1;
        }
        b
    }

    pub fn as_slice(&self) -> &[Band] {
        &self.at[..self.n]
    }

    /// Row `y` changed. Rows come in rising order; one below every band
    /// it may join is a band of its own, and with no room left it widens
    /// the last band down to it.
    pub fn add(&mut self, y: usize) {
        if let Some(last) = self.n.checked_sub(1).map(|i| &mut self.at[i]) {
            if y < last.end() {
                return;
            }
            if y <= last.end().saturating_add(JOIN_GAP) || self.n == MAX_BANDS {
                last.rows = y.saturating_add(1).saturating_sub(last.y);
                return;
            }
        }
        self.at[self.n] = Band { y, rows: 1 };
        self.n += 1;
    }
}
