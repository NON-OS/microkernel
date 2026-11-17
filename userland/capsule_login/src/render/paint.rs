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

//! Painting the lock screen in the theme chosen in Settings.

use nonos_app_skeleton::runner::follow_theme;
use nonos_toolkit::theme::roles;

use super::locked::{draw_clear, draw_locked};
use crate::state::Context;

/// Covers the screen at once in the theme held, then reads the chosen theme
/// and draws again only if it changed, so a slow policy store never delays
/// the lock.
pub fn paint_locked(ctx: &Context) {
    draw_locked(ctx, &roles());
    if follow_theme() {
        draw_locked(ctx, &roles());
    }
}

/// No lock showing: an unlocked session, or none started yet.
pub fn paint_clear(ctx: &Context) {
    draw_clear(ctx);
}

pub fn paint_unlocked(ctx: &Context) {
    paint_clear(ctx);
}
