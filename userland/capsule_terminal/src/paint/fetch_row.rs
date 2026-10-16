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

//! The splash panel's measures, and how one labelled fact row is drawn.

use nonos_app_skeleton::PaintBuffer;

use crate::term::theme::types::Theme;

// Breathing room between the titlebar and the top of the banner. Anchored to
// the body rect so the art starts inside the content area, not butted against
// the chrome above it.
pub(super) const BANNER_PAD: i32 = 14;
pub(super) const RULE_W: i32 = 300;
pub(super) const INFO_PX: f32 = 14.0;
pub(super) const ROW: i32 = 20;

pub(super) fn row(fb: &mut PaintBuffer, x: i32, y: i32, label: &str, value: &str, t: &Theme) {
    let _ = fb.text_ttf_mono(x, y, label, t.accent, INFO_PX);
    let _ = fb.text_ttf_mono(x + 96, y, value, t.fg, INFO_PX);
}
