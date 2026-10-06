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

//! The brand panel: the glowing Ø in its frame, as on nonos.software, and
//! under it what this screen is and which release.

use super::layout::Layout;
use crate::display::ink::palette::{CYAN, TEXT_2, TEXT_3};
use crate::display::ink::{draw_captions as captions, draw_emblem as emblem};
use crate::display::version::version_label;

/// The frame drawn to `progress` thousandths, and the Ø once it is whole.
pub(super) fn draw_emblem(l: &Layout, progress: u32) {
    emblem(&l.s, progress, CYAN, true);
}

pub(super) fn draw_captions(l: &Layout) {
    captions(&l.s, b"VERIFIED BOOT", TEXT_2, version_label().as_bytes(), TEXT_3);
}
