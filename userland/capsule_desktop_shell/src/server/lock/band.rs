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

//! The band of the lock screen that a clock tick or a key in its field
//! changes: the time, the date, the field and the note below it, over every
//! output the curtain is on (lock::geometry::tick_bounds). The frame is drawn
//! whole as ever and presented with the band alone named.

use crate::compositor_client::push_damage_commit;
use crate::render::ui_font::{line_h, px, UI_PX};
use crate::render::{paint_chrome_in, Changed};
use crate::state::lock::geometry::tick_bounds;
use crate::state::Context;

/// The band, as x, y, width, height.
pub fn band(ctx: &Context) -> (u32, u32, u32, u32) {
    let note_h = 4 * line_h(UI_PX);
    let whole = (0, 0, ctx.width, ctx.height);
    tick_bounds(&ctx.lock.outputs, whole, px(1), note_h)
}

/// A key in the field shown: the band alone, committed only when its frame
/// changed and the compositor did not take it. Toasts raised or gone since
/// the last frame are not in the band, so the frame is shown whole then.
pub(super) fn show(ctx: &mut Context) {
    if ctx.toast_view.toasts_drawn != ctx.toasts.generation() {
        return crate::server::repaint::repaint(ctx);
    }
    let r = band(ctx);
    // A present the compositor took damaged the band itself (render/chrome/sent.rs).
    for &(x, y, w, h) in paint_chrome_in(ctx, Changed::Part(r.0, r.1, r.2, r.3)).owed().of(&[r]) {
        let rid = ctx.issue_request_id();
        let _ = push_damage_commit(ctx.ports.compositor_port, rid, x, y, w, h);
    }
}
