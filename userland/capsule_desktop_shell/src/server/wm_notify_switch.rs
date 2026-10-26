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

//! A window notice, for the window switcher's order: a window focused goes
//! to its front, one opened behind it, one closed out, and a strip on screen
//! shows the change. The shell's own windows are never switched to.

use nonos_libc::{mk_getpid, mk_uptime_ms};

use crate::state::wm_notice::WmEvent;
use crate::state::Context;

pub(super) fn note(ctx: &mut Context, owner_pid: u32, window_id: u32, event: WmEvent) {
    // Pid 0 is the window manager saying no window has focus.
    if owner_pid == 0 || owner_pid == mk_getpid() {
        return;
    }
    let w = (owner_pid, window_id);
    match event {
        WmEvent::Focused => ctx.switch.focused(w),
        WmEvent::Opened => ctx.switch.opened(w),
        WmEvent::Closed => {
            ctx.switch.closed(w);
            if ctx.switch.session.as_ref().is_some_and(|s| s.live(mk_uptime_ms())) {
                super::repaint::repaint(ctx);
            }
        }
        WmEvent::FullScreen(_) => {}
    }
}
