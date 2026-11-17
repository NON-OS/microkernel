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

//! One binding: what it does, then its key.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Ink, Rect, Row};

use super::super::text;

/// A shared row: what the binding does, with its key at the right in the
/// accent, as a menu shows a shortcut.
pub fn paint(fb: &mut PaintBuffer, at: Rect, key: &[u8], label: &[u8]) {
    Row::new(text::valid(label)).value(text::valid(key)).value_ink(Ink::Accent).draw(fb, at);
}
