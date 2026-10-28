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

//! Following each install or uninstall this Terminal queued until it ends,
//! then saying how on a line of its own: it once said "queued" and nothing
//! more, even of an installer that had stopped partway.

use alloc::format;
use alloc::string::String;
use core::cell::UnsafeCell;

use nonos_libc::{mk_app_install_status, mk_time_millis};
use nonos_market_proto::Stage;

use super::stage_text::stage_text;
use super::watch::Watch;
use crate::term::scrollback::Scrollback;

struct Kept(UnsafeCell<Watch<bool>>);

// Safety: reached only from the event loop's thread, by asked and tick,
// neither of which holds a reference past its return.
unsafe impl Sync for Kept {}

static WATCH: Kept = Kept(UnsafeCell::new(Watch::new()));

/// The system's answer `rc` to asking for `id`, followed when it queued it.
/// `removing` is true for an uninstall.
pub(super) fn asked(id: &[u8], removing: bool, rc: i64) -> i64 {
    if rc == 0 {
        unsafe { (*WATCH.0.get()).add(id, removing) }
    }
    rc
}

/// Once a second at most, say each followed listing that has finished.
/// True when a line was added. Held while a program has the screen.
pub fn tick(sb: &mut Scrollback, held: bool) -> bool {
    let watch = unsafe { &mut *WATCH.0.get() };
    if held {
        return false;
    }
    let ask = |id: &[u8]| Stage::of(mk_app_install_status(id));
    let ended = watch.due(|| mk_time_millis(), ask, |s| !s.pending() && s != Stage::Idle);
    for (id, removing, stage) in &ended {
        let line = said(id, *removing, *stage);
        match stage {
            Stage::Installed | Stage::Removed => sb.push_line(line.as_bytes()),
            _ => sb.push_error(line.as_bytes()),
        }
    }
    !ended.is_empty()
}

fn said(id: &[u8], removing: bool, stage: Stage) -> String {
    let name = String::from_utf8_lossy(id);
    let what = if removing { "uninstall" } else { "install" };
    match stage {
        Stage::Failed(_) => format!("market: {name}: {what} failed: {}", stage_text(id, stage)),
        _ => format!("market: {name}: {}", stage_text(id, stage)),
    }
}
