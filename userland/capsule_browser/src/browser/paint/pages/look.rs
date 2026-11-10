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

//! The colours the browser's own pages are painted in: the OS theme's, the
//! ones the shared widgets drawn on them take, under every appearance. The
//! Light and Dark appearances tint only the start page and the web pages.

use nonos_toolkit::theme::{roles, Category, Roles};

/// The ground the own pages are painted on under the roles `r`. Pure, so the
/// proofs can check it in every scheme.
pub fn ground_in(r: &Roles) -> u32 {
    r.bg.argb()
}

/// The same under the theme in force.
pub fn ground() -> u32 {
    ground_in(&roles())
}

/// The fill of category `k` and the ink that reads on it, in the theme in force.
pub fn category(k: Category) -> (u32, u32) {
    (k.colour().argb(), k.ink().argb())
}
