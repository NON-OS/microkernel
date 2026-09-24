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

//! The live theme, and the two roles derived from it.

use core::sync::atomic::AtomicU32;

use super::super::palette;

pub static BG: AtomicU32 = AtomicU32::new(palette::BACKGROUND);
pub static SURFACE: AtomicU32 = AtomicU32::new(palette::SURFACE);
pub static ACCENT: AtomicU32 = AtomicU32::new(palette::ACCENT);
pub static TEXT: AtomicU32 = AtomicU32::new(palette::TEXT);
pub static BORDER: AtomicU32 = AtomicU32::new(palette::BORDER);
pub static REVISION: AtomicU32 = AtomicU32::new(1);

/*
 * Muted and quiet are derived, and derived by walking a contrast floor, so they are
 * cached here rather than recomputed. Text is drawn hundreds of times a frame and
 * the walk is a loop over luminance sums; doing it per glyph run would put a
 * contrast search inside the paint path for a value that only changes when the
 * theme does.
 *
 * The defaults match what `replace` would compute for the palette above, so a
 * capsule that never reaches the settings store still gets a legible secondary
 * label rather than a zero.
 */
pub static MUTED: AtomicU32 = AtomicU32::new(palette::MUTED);
pub static QUIET: AtomicU32 = AtomicU32::new(palette::DISABLED);
