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

//! The toolkit's categories, lifted for the start page's grounds.

use nonos_toolkit::theme::Category;

use super::{palette, Palette};

/// The fill of category `k` and the ink that reads on it, under the palette
/// in force: lifted against the pages' grounds, so the Light and Dark
/// appearances get colours that read on them and not on the OS theme's.
pub fn category(k: Category) -> (u32, u32) {
    category_on(&palette(), k)
}

/// The same under the palette `p`. Pure, so the proofs can check it under
/// every appearance.
pub fn category_on(p: &Palette, k: Category) -> (u32, u32) {
    let grounds = [p.ground, p.surface, p.sidebar, p.field];
    (k.colour_over(&grounds, p.ink).argb(), k.ink_over(&grounds, p.ink).argb())
}
