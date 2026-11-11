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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Rect, Separator};

use super::layout::{font, hair, metrics, px};
use super::sel_summary::count_label;
use super::state::{Mode, State};

const HINT: &str = "n new    m dir    r rename    d del    c/x/p copy    / find    ? help";
const FOOT_PX: f32 = 14.0;
const GAP: u32 = 10;

/// The status line and the hint strip share one band, and both are measured off
/// the same facade the rest of the chrome draws with, so a long status pushes
/// the caret exactly as far as the glyphs actually ran.
pub fn paint_footer(state: &State, fb: &mut PaintBuffer) {
    let (w, r, m, foot_px, gap) = (fb.width, roles(), metrics(), font(FOOT_PX), px(GAP));
    let (band, x0) = (w.saturating_sub(m.sidebar_w), m.sidebar_w);
    let y = fb.height.saturating_sub(m.footer_h);
    draw::fill(fb, Rect { x: x0, y, w: band, h: m.footer_h }, r.sunken);
    Separator::new().draw(fb, Rect { x: x0, y, w: band, h: hair() });
    let mut left = x0 + m.pad_x;
    let ty = (y + px(8)) as i32;
    if let Some(count) = count_label(state) {
        let _ = fb.text_ttf(left as i32, ty, count.as_str(), r.accent.argb(), foot_px);
        left += text::width(count.as_str(), foot_px) + gap;
    }
    let status = core::str::from_utf8(state.status).unwrap_or("");
    let _ = fb.text_ttf(left as i32, ty, status, r.muted.argb(), foot_px);
    match state.mode {
        Mode::Prompt(_) | Mode::Filter => {
            let typed = if matches!(state.mode, Mode::Filter) {
                state.filter.as_str()
            } else {
                state.input.as_str()
            };
            let x = left + text::width(status, foot_px) + gap;
            let _ = fb.text_ttf(x as i32, ty, typed, r.text.argb(), foot_px);
        }
        _ => {
            let hw = text::width(HINT, foot_px);
            if w > x0 + m.pad_x * 2 + hw {
                let hx = w.saturating_sub(m.pad_x + hw) as i32;
                let _ = fb.text_ttf(hx, ty, HINT, r.quiet.argb(), foot_px);
            }
        }
    }
}
