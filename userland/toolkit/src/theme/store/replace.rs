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

//! Taking a new theme.

use core::sync::atomic::Ordering;

use super::state::{ACCENT, BG, BORDER, MUTED, QUIET, REVISION, SURFACE, TEXT};
use super::theme::Theme;

pub fn replace(new: Theme) {
    BG.store(new.background_argb, Ordering::Release);
    SURFACE.store(new.surface_argb, Ordering::Release);
    ACCENT.store(new.accent_argb, Ordering::Release);
    TEXT.store(new.text_argb, Ordering::Release);
    BORDER.store(new.border_argb, Ordering::Release);
    MUTED.store(new.muted_argb, Ordering::Release);
    QUIET.store(new.quiet_argb, Ordering::Release);
    /*
     * Last, and after the colours it describes. A reader that saw a new revision
     * would repaint, and repainting between the stores would draw half of one theme
     * and half of another.
     */
    REVISION.fetch_add(1, Ordering::AcqRel);
}
