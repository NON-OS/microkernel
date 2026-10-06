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

//! Say each launch whose window did not come (state/taskbar/expect.rs).

use nonos_app_skeleton::log_line::{say as log, Line};

use crate::state::says::{named, NO_WINDOW};
use crate::state::toast::TOAST_TEXT_MAX;
use crate::state::{windows_overdue, Context, NotifyLevel, Uptime, LAUNCHER_APPS};

pub fn say(ctx: &mut Context, now: Uptime) {
    let late = windows_overdue(&mut ctx.taskbar, now);
    if late == 0 {
        return;
    }
    for (index, app) in LAUNCHER_APPS.iter().enumerate() {
        if late & 1u64.checked_shl(index as u32).unwrap_or(0) == 0 {
            continue;
        }
        let mut line = [0u8; TOAST_TEXT_MAX];
        let n = named(app.label, NO_WINDOW, &mut line);
        ctx.toasts.push(&line[..n], NotifyLevel::Error, crate::server::toast_clock::now());
        /* The spawn was queued; init's answer to it is `log SPAWN-INSTANCE`,
         * and an app that opened no window says why under `log app-fail`. */
        let _ = log(&Line::new(b"LAUNCH")
            .text(&line[..n])
            .text(b"; see log spawn-instance, log app-fail"));
    }
}
