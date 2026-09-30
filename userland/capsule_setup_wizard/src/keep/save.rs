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
use nonos_policy_proto::setup_record::{Answers, ANSWERS_PATH, DONE, DONE_PATH, SETUP_DIR};

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
    };
    let pid = mk_getpid();
    let _ = vfs::mkdir(pid, b"/nonos");
    let _ = vfs::mkdir(pid, SETUP_DIR);
    match put(pid, ANSWERS_PATH, &answers.encode()).and_then(|()| put(pid, DONE_PATH, &DONE)) {
        Ok(()) => say(b"[SETUP] answers kept; the next boot skips setup\n"),
        Err(why) => {
            say(b"[SETUP] answers not kept, setup runs again next boot: ");
            say(why.as_bytes());
            say(b"\n");
        }
    }
}

/*
 * A record loaded from an earlier boot belongs to nobody, and only a file's
 * owner may persist it, so it is unlinked and written afresh first.
 */
fn put(pid: u32, path: &[u8], bytes: &[u8]) -> Result<(), &'static str> {
    let _ = vfs::unlink(pid, path);
    vfs::write_file(pid, path, bytes)?;
    vfs::persist(pid, path)
}
