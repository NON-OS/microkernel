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

use core::sync::atomic::{AtomicU8, Ordering};

use super::layout::splash;
use crate::display::fx::clear_region;
use crate::display::gop::{get_dimensions, hline, is_initialized};
use crate::display::ink::palette::{BORDER, CYAN, TEXT_2, TEXT_3};
use crate::display::ink::{label, label_width, metrics, round_rect, Style};
use crate::display::text::Text;
use crate::display::version::version_label;

static CURRENT_STAGE: AtomicU8 = AtomicU8::new(0);
static TOTAL_STAGES: AtomicU8 = AtomicU8::new(10);

pub fn draw_boot_progress(current: u8, total: u8) {
    CURRENT_STAGE.store(current, Ordering::Relaxed);
    TOTAL_STAGES.store(total, Ordering::Relaxed);
    render_progress_bar();
}

pub fn get_progress() -> (u8, u8) {
    (CURRENT_STAGE.load(Ordering::Relaxed), TOTAL_STAGES.load(Ordering::Relaxed))
}

/// The footer, as on the menu: the stage count with a cyan line that grows,
/// and the release on the right.
fn render_progress_bar() {
    if !is_initialized() {
        return;
    }
    let s = splash();
    let (w, h) = get_dimensions();
    let (cur, total) = get_progress();
    let (u, mono) = (s.u, metrics(Style::Mono));
    let (side, y) = (8 * u, s.footer_y + mono.line + 2 * u);
    clear_region(0, s.footer_y, w, h.saturating_sub(s.footer_y));
    hline(0, s.footer_y, w, BORDER);
    let t = Text::new().push(b"STEP ").dec(cur as u64).push(b" OF ").dec(total as u64);
    let tw = label(side, y, t.as_bytes(), TEXT_2);
    let span = tw.max(label_width(b"STEP 10 OF 10"));
    round_rect(side, y + mono.line + u, span, 2, 1, BORDER);
    round_rect(side, y + mono.line + u, (span * cur as u32 / total.max(1) as u32).max(2), 2, 1, CYAN);
    let v = version_label();
    label(w.saturating_sub(side + label_width(v.as_bytes())), y, v.as_bytes(), TEXT_3);
}
