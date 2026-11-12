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

use nonos_toolkit::design::scale::scale;

use crate::browser::layout::boxmodel::device_px;
use crate::browser::state::State;

const TOP: i32 = 0;
const PAGE_BG: u32 = 0xFF18_1B20;
const FG: u32 = 0xFFE8_EAED;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let bottom = fb.height as i32;
    fb.fill_rect(0, TOP as u32, fb.width, fb.height.saturating_sub(TOP as u32), PAGE_BG);
    let Some(doc) = &state.document else {
        fb.text_ttf(16, TOP + 24, &state.status, FG, 15.0);
        return;
    };
    let d = |v: u32| device_px(v.min(i32::MAX as u32) as i32);
    for line in &doc.lines {
        let (top, lh) = (d(line.y), d(line.y + line.height) - d(line.y));
        let sy = top + TOP - state.scroll as i32;
        if sy < TOP || sy + lh > bottom {
            continue;
        }
        for s in &line.spans {
            let (x, w) = (d(s.x), d(s.x + s.w) - d(s.x));
            if s.bg != 0 {
                fb.fill_rect(x as u32, sy as u32, w as u32, lh as u32, s.bg);
            }
            let px = scale().font(crate::browser::layout::px_for(s.scale));
            fb.text_ttf(x, sy, &s.text, s.color, px);
            if s.bold {
                fb.text_ttf(x + scale().hair() as i32, sy, &s.text, s.color, px);
            }
            if s.href.is_some() {
                let under = (sy + lh) as u32 - scale().px(4);
                fb.fill_rect(x as u32, under, w as u32, scale().hair(), s.color);
            }
        }
    }
}
