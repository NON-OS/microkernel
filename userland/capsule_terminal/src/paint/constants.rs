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

use nonos_toolkit::design::scale::scale;

const TEXT_LEFT: u32 = 14;
const BODY_PAD_TOP: u32 = 6;

/// The body's text inset from its left edge, in drawing pixels.
pub fn text_left() -> u32 {
    scale().px(TEXT_LEFT)
}

/// The gap between the header and the body, in drawing pixels.
pub fn body_pad_top() -> u32 {
    scale().px(BODY_PAD_TOP)
}
