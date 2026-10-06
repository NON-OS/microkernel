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

//! Say it out loud, once, if the capsule store failed to decode at boot. The
//! probe retries until vfs_pool answers, then latches: a corrupted store used
//! to present as a merely empty /capsules with nothing said.

use core::sync::atomic::{AtomicBool, Ordering};

use nonos_app_skeleton::log_line::{say as log, Line};

use super::backoff::Backoff;
use crate::render::sync_toast_layer;
use crate::state::{Context, NotifyLevel};

static ANSWERED: AtomicBool = AtomicBool::new(false);

/// Same window, same reason: vfs_pool cannot answer while it stages packages.
static GAP: Backoff = Backoff::new(250, 4000);

pub fn check(ctx: &mut Context) {
    if ANSWERED.load(Ordering::Relaxed) {
        return;
    }
    if !GAP.due() {
        return;
    }
    let Some((code, settled)) = crate::vfs_client::store_status() else {
        GAP.missed();
        return;
    };
    // While staging is in flight the block device may not be ready, so the first
    // attempt records a transient error that the next one clears. Wait for the
    // definite answer rather than latching a false "corrupted".
    if !settled {
        GAP.missed();
        return;
    }
    ANSWERED.store(true, Ordering::Relaxed);
    let Some((text, level)) = crate::state::store_word::store_word(code) else {
        /* A fault the desktop has no words for (state/store_word.rs says
         * which have them) would otherwise pass in silence: the log names
         * its code, which `log shell` shows beside the rest of the boot. */
        if code != 0 {
            let line = Line::new(b"SHELL").text(b"store: loaded with fault, vfs store status ");
            let _ = log(&line.num(code.into()).text(b"; nothing said on screen for it"));
        }
        return;
    };
    /* A warning is held so it is not missed; no disk at all (an ISO, a live
     * boot) is only a notice, as long as any other. */
    let now = crate::server::toast_clock::now();
    match level {
        NotifyLevel::Info => ctx.toasts.push(text, level, now),
        _ => ctx.toasts.push_held(text, level, now),
    }
    sync_toast_layer(ctx);
    let line = Line::new(b"SHELL").text(b"store: ").text(text);
    let _ = log(&line.text(b" (vfs store status ").num(code.into()).text(b")"));
}
