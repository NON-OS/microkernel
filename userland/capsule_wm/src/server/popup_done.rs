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

use nonos_libc::{mk_ipc_send_to_pid, mk_yield};

use crate::protocol::notify_send::offer;
use crate::protocol::{encode_notify, NOTIFY_KIND_POPUP_DONE, NOTIFY_LEN};
use crate::state::Context;
use crate::window::table::PER_OWNER;

/// Close `owner_pid`'s popups over window `window_id`, and the popups over
/// those, telling the project of each, the topmost first. With `itself`,
/// `window_id` (a popup) closes too and is told of the same way.
pub fn close_popups(ctx: &mut Context, owner_pid: u32, window_id: u32, itself: bool) {
    let mut closed = [0u32; PER_OWNER];
    let n = ctx.windows.remove_popups(owner_pid, window_id, itself, &mut closed);
    for &id in closed[..n].iter() {
        let mut frame = [0u8; NOTIFY_LEN];
        encode_notify(&mut frame, NOTIFY_KIND_POPUP_DONE, owner_pid, id, 0, 0);
        let send = || mk_ipc_send_to_pid(owner_pid, frame.as_ptr(), frame.len());
        let _ = offer(send, || {
            let _ = mk_yield();
        });
    }
}
