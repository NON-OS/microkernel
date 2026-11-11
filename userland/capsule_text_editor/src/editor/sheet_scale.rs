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

//! The page is laid out once, in logical pixels: the units the model, the
//! measurer and the exports share, so a document breaks onto the same pages at
//! every display scale. The canvas, its caret and its hit test carry a page
//! length onto the screen through here, so each is scaled exactly once.

use nonos_toolkit::design::scale::scale;

/// A page length, in drawing pixels.
pub(super) fn on_screen(page: f32) -> f32 {
    scale().font(page)
}

/// A drawing-pixel length, back on the page.
pub(super) fn on_page(screen: f32) -> f32 {
    screen * 4.0 / scale().quarters() as f32
}
