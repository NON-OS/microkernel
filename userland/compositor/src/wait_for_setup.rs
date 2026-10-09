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

use alloc::format;

use nonos_libc::{mk_uptime_ms, mk_yield};

use crate::say::{say, say_display};
use crate::state::Context;

// How long a virtio-gpu driver gets to announce its gfx service before the
// GOP framebuffer is taken, so a machine that has virtio-gpu uses it and
// real hardware takes the GOP route. On the uptime clock: six rounds of 64
// yields lasted however long the yields took, and that changes with the
// number of cores. A driver that announces later still gets the display
// through upgrade_to_virtio.
const VIRTIO_WAIT_MS: i64 = 500;
// Between two asks, so a machine with neither display does not spin.
const RETRY_MS: i64 = 20;

pub fn wait_for_setup() -> Context {
    let start = mk_uptime_ms();
    let mut said_none = false;
    loop {
        if let Ok(ctx) = crate::setup::run_virtio() {
            return fitted(ctx, "virtio-gpu");
        }
        if mk_uptime_ms().saturating_sub(start) >= VIRTIO_WAIT_MS {
            match crate::setup::run_gop() {
                Ok(ctx) => return fitted(ctx, "GOP framebuffer"),
                Err(why) if !said_none => {
                    say(&format!("no display yet ({}); waiting for a virtio-gpu driver", why));
                    said_none = true;
                }
                Err(_) => {}
            }
        }
        let next = mk_uptime_ms().saturating_add(RETRY_MS);
        while mk_uptime_ms() < next {
            mk_yield();
        }
    }
}

fn fitted(mut ctx: Context, path: &str) -> Context {
    crate::setup::fit_canvas(&mut ctx);
    say_display(&ctx, path);
    ctx
}
