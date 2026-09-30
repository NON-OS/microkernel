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

//! Keeping the answers, when the review screen commits.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;
use nonos_policy_proto::setup_record::{Answers, Name, Record, Tier, DONE, DONE_PATH, SETUP_DIR};

use super::put::{put, put_answers};

use crate::render::screens::{appearance, keyboard};
use crate::server::say::say;
use crate::state::Context;

/// The answers first and the marker last: the policy service restores only
/// beside a marker, so a save cut short restores nothing and setup runs again.
pub fn save(ctx: &Context) {
    let answers = Answers {
        keyboard_layout: keyboard::layout(ctx.kbd_sel),
        timezone: ctx.tz_off,
        wallpaper: appearance::wallpaper(ctx.wall_sel),
        /* The name step and the tier table take nothing these refuse. */
        username: Name::new(ctx.name.typed()).unwrap_or(Name::EMPTY),
        qwen_tier: ctx.qwen.chosen().and_then(Tier::new).unwrap_or(Tier::EMPTY),
    };
    let pid = mk_getpid();
    let _ = vfs::mkdir(pid, b"/nonos");
    let _ = vfs::mkdir(pid, SETUP_DIR);
    let record = Record { answers, apps_off: ctx.apps_off };
    match put_answers(pid, &record).and_then(|()| put(pid, DONE_PATH, &DONE)) {
        Ok(()) => say(b"[SETUP] answers kept; the next boot skips setup\n"),
        Err(why) => {
            say(b"[SETUP] answers not kept, setup runs again next boot: ");
            say(why.as_bytes());
            say(b"\n");
        }
    }
}
