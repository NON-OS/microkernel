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

//! The scaling the desktop asks of the wallpaper (Fill), sent once the
//! wallpaper service answers. Setup tries it once and goes on either way; the
//! runner's clock tick tries again until it is through, so neither a slow nor
//! a missing wallpaper keeps the desktop from coming up.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_app_skeleton::log_line::{say as log, Line};

use super::backoff::Backoff;
use crate::state::Context;
use crate::wallpaper_client;

/// Between tries: each call may wait out its 250 ms reply budget on a busy
/// wallpaper, so the shell asks less often while it stays busy.
static GAP: Backoff = Backoff::new(250, 8000);

/// Policy::Fill. The image itself is chosen by the Wallpaper policy field.
const FILL: u32 = 3;

/// Tries that did not go through, for the log: the first miss is said with
/// its reason, and the try that then goes through says how many it took.
/// Never a line per try: the runner asks again every few seconds.
static MISSED: AtomicU32 = AtomicU32::new(0);

/// Send the scaling to the wallpaper at `port`; true once it took it.
pub fn send(port: u32) -> bool {
    let sent = match port {
        0 => Err("no wallpaper service yet"),
        _ => wallpaper_client::queue_policy(port, FILL, 0),
    };
    let missed = MISSED.load(Ordering::Relaxed);
    match sent {
        /* An answer that refuses is an answer: asking again would get the
         * same one, so it is said and not asked again. */
        Ok(status) if status != 0 => {
            let line = Line::new(b"SHELL").text(b"wallpaper: scaling refused, status ");
            let _ = log(&line.num(status.into()).text(b"; the wallpaper keeps its own"));
        }
        Ok(_) if missed > 0 => {
            MISSED.store(0, Ordering::Relaxed);
            let line = Line::new(b"SHELL").text(b"wallpaper: scaling taken after ");
            let _ = log(&line.num(missed.into()).text(b" missed tries"));
        }
        Ok(_) => {}
        Err(why) => {
            if missed == 0 {
                let line = Line::new(b"SHELL").text(b"wallpaper: scaling not taken: ");
                let _ = log(&line.text(why.as_bytes()).text(b"; asking again"));
            }
            MISSED.store(missed.saturating_add(1), Ordering::Relaxed);
        }
    }
    sent.is_ok()
}

/// From the runner: find the wallpaper and send the scaling, until done.
pub fn retry(ctx: &mut Context) {
    if ctx.wallpaper_policy_sent || !GAP.due() {
        return;
    }
    if ctx.wallpaper_port == 0 {
        ctx.wallpaper_port = crate::setup::try_wallpaper();
    }
    ctx.wallpaper_policy_sent = send(ctx.wallpaper_port);
    if !ctx.wallpaper_policy_sent {
        GAP.missed();
    }
}
