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

//! A card's buttons: the one its stage offers, and Remove once installed.

use nonos_app_skeleton::PaintBuffer;

use crate::store::listing::Listing;
use crate::store::progress::Progress;
use crate::store::theme::{
    ACCENT, BUTTON_BG, BUTTON_FG, BUTTON_OFF_BG, BUTTON_OFF_FG, DANGER, MUTED,
};

use super::super::metrics::SMALL_PX;
use super::super::text;

pub(super) fn action(fb: &mut PaintBuffer, l: &Listing, (x, y, w, h): (u32, u32, u32, u32)) {
    let word = l.progress.button(l.ready);
    let (bg, fg) = match l.progress {
        p if p.uninstalled() && l.ready => (BUTTON_BG, BUTTON_FG),
        Progress::Installed => (BUTTON_BG, BUTTON_FG),
        Progress::Queued | Progress::Installing | Progress::Removing => (BUTTON_OFF_BG, ACCENT),
        p if p.needs_installed_system() => (BUTTON_OFF_BG, MUTED),
        p if p.retryable() => (BUTTON_OFF_BG, DANGER),
        Progress::Refused | Progress::Failed(_) => (BUTTON_OFF_BG, BUTTON_OFF_FG),
        Progress::Idle | Progress::Removed => (BUTTON_OFF_BG, BUTTON_OFF_FG),
    };
    fb.fill_rect(x, y, w, h, bg);
    let tx = x + (w - text::width_of(word, SMALL_PX)) / 2;
    text::line(fb, tx, text::top_of(y as i32, h, SMALL_PX), word, fg, SMALL_PX);
}

pub(super) fn remove(fb: &mut PaintBuffer, (rx, ry, rw, rh): (u32, u32, u32, u32)) {
    fb.fill_rect(rx, ry, rw, rh, BUTTON_OFF_BG);
    let word: &[u8] = b"Remove";
    let tx = rx + (rw - text::width_of(word, SMALL_PX)) / 2;
    text::line(fb, tx, text::top_of(ry as i32, rh, SMALL_PX), word, BUTTON_OFF_FG, SMALL_PX);
}
