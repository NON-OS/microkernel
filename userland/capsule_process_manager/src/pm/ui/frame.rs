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

//! The card shape, on its own.

use nonos_app_skeleton::PaintBuffer;

use crate::pm::theme::{CARD_BG, CARD_BORDER};

use super::metrics::CARD_RADIUS;

// The rounded rect on its own. A card whose body will not fit the shared
// padding draws this and lays itself out, which keeps its outer edge identical
// to its neighbours while its inside is its own business.
pub fn frame(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    fb.fill_round(x, y, w, h, CARD_RADIUS, CARD_BG);
    fb.stroke_round(x, y, w, h, CARD_RADIUS, 1, CARD_BORDER);
}
