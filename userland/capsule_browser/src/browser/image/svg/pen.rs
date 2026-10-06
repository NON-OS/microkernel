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

/// Stroke geometry, inherited like the other stroke properties: width,
/// cap (0 butt, 1 round, 2 square), join (0 miter, 1 round, 2 bevel),
/// miter limit, dash lengths and dash offset.
#[derive(Clone, Copy)]
pub(super) struct Pen {
    pub width: f32,
    pub cap: u8,
    pub join: u8,
    pub miter: f32,
    pub dash: [f32; 16],
    pub dashes: usize,
    pub offset: f32,
}

impl Pen {
    pub fn initial() -> Pen {
        Pen { width: 1.0, cap: 0, join: 0, miter: 4.0, dash: [0.0; 16], dashes: 0, offset: 0.0 }
    }

    /// The pen in device pixels, for a user-to-device scale of `k`.
    pub fn scaled(&self, k: f32) -> Pen {
        let mut p = *self;
        p.width *= k;
        p.offset *= k;
        p.dash.iter_mut().for_each(|d| *d *= k);
        p
    }
}
