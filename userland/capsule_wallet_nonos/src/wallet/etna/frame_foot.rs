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

//! The fixed foot of the frame: the pinned actions and the status line.

use nonos_app_skeleton::PaintBuffer;

use super::frame_spec::{FrameLayout, FrameSpec};
use super::parts::action::action;
use super::parts::rule::rule;
use super::parts::status::status_line;
use super::rect::Rect;
use super::tokens::{COLUMN, INK, ROOM, SIDE, STATUS_H, TALL, TIGHT};

fn buttons_height(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        ROOM * 2 + TALL * n + TIGHT * (n - 1) + 1
    }
}

pub fn foot_height(spec: &FrameSpec) -> u32 {
    let status = if spec.status.is_empty() { 0 } else { STATUS_H };
    buttons_height(spec.footer.len() as u32) + status
}

pub fn foot(fb: &mut PaintBuffer, x0: u32, spec: &FrameSpec, out: &mut FrameLayout) {
    let top = fb.height.saturating_sub(foot_height(spec));
    fb.fill_rect(x0, top, COLUMN, fb.height - top, INK);
    let n = spec.footer.len() as u32;
    if n > 0 {
        rule(fb, x0, top, COLUMN, 0);
        for (i, (title, weight, enabled)) in spec.footer.iter().enumerate() {
            let at = Rect::new(
                x0 + SIDE,
                top + 1 + ROOM + (TALL + TIGHT) * i as u32,
                COLUMN - 2 * SIDE,
                TALL,
            );
            action(fb, at, title, *weight, *enabled);
            out.footer[i.min(2)] = at;
        }
    }
    if !spec.status.is_empty() {
        let sy = fb.height - STATUS_H;
        rule(fb, x0, sy, COLUMN, 0);
        status_line(fb, x0, sy + 1, spec.status);
    }
}
