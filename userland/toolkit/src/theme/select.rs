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

//! Turning two stored settings into the live theme.

use super::schemes::{scheme, HIGH_CONTRAST};
use super::store::Theme;

/// The theme for a stored theme index and contrast preference.
///
pub fn theme_of(index: u8, high_contrast: bool) -> Theme {
    let s = if high_contrast { HIGH_CONTRAST } else { scheme(index) };
    Theme::from_roles(s.bg, s.surface, s.accent, s.text, s.border)
}
