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
use nonos_toolkit::theme::{roles, Opaque, Roles};
use nonos_toolkit::widgets::text;

use crate::settings::state::{State, StatusKind};

use super::bytes::as_str;
use super::metrics::metrics;

const BAR_H: u32 = 34;

/// The strip only exists when it has something to report, so a healthy panel
/// looks like the design and a failing one still says why.
pub fn status_h(state: &State) -> u32 {
    if state.policy_ready && state.status.kind == StatusKind::Idle {
        0
    } else {
        scale().px(BAR_H)
    }
}

pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let h = status_h(state);
    if h == 0 {
        return;
    }
    let (r, m) = (roles(), metrics());
    let (y, x) = (state.win_h.saturating_sub(h), m.sidebar_w);
    let w = state.win_w.saturating_sub(x);
    fb.fill_rect(x, y, w, h, r.sunken.argb());
    fb.fill_rect(x, y, w, scale().hair(), r.border.argb());
    let (fg, msg) = message(state, r);
    let top = text::centre_top(y, h, m.body_px);
    text::left(fb, x + m.pane_pad_x, top, msg, fg, m.body_px);
}

fn message(state: &State, r: Roles) -> (Opaque, &str) {
    if !state.policy_ready {
        return (r.warn, "Policy service unavailable - values shown are not stored");
    }
    let fg = match state.status.kind {
        StatusKind::Error => r.warn,
        _ => r.ok,
    };
    (fg, as_str(state.status.as_slice()))
}
