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

use crate::render::layout::bottom_dock_rect;
use crate::state::{Context, TASKBAR_WINDOW_ID};
use crate::wm_client;
use nonos_libc::mk_idle_ms;

const WINDOW_KIND_POPUP: u32 = 3;
/* The dock's own window, tried as an app's is (app_skeleton
 * setup/patience.rs): rested between tries, not yielded. A bare yield comes
 * back at once when nothing else wants the processor, so sixteen of them
 * were spent in microseconds on a window manager that was only a frame
 * behind, and the whole desktop setup went round again. */
const OPEN_RETRIES: u32 = nonos_app_skeleton::setup::patience::WINDOW_OPEN.attempts * 4;
const REST_MS: u64 = nonos_app_skeleton::setup::patience::WINDOW_OPEN.rest_ms;

pub fn open_chrome_windows(ctx: &mut Context) -> Result<(), &'static str> {
    let bottom = bottom_dock_rect(ctx.width, ctx.height);
    open_retry(ctx, TASKBAR_WINDOW_ID, bottom.x, bottom.y, bottom.width, bottom.height)
        .map_err(|_| "wm rejected taskbar window_open")
}

fn open_retry(
    ctx: &mut Context,
    window_id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<(), &'static str> {
    for attempt in 0..OPEN_RETRIES {
        if wm_client::window_open(
            ctx.wm_port,
            ctx.issue_request_id(),
            window_id,
            WINDOW_KIND_POPUP,
            x,
            y,
            width,
            height,
        )
        .is_ok()
        {
            return Ok(());
        }
        if attempt + 1 < OPEN_RETRIES {
            mk_idle_ms(REST_MS);
        }
    }
    Err("wm rejected window_open")
}
