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
//! Tell lifecycle subscribers which window has focus whenever it changed
//! Tell a window's owner, and no one else, that its window took or lost the
//! keyboard (NOTIFY_KIND_KEYS), so it can draw itself focused or not. The
//! broadcast to lifecycle subscribers is focus_notify's; this one names only
//! the owner's own window, so no app learns which other app is in front.

use nonos_libc::{mk_ipc_send_to_pid, mk_yield};

use crate::focus::model::FocusedRef;
use crate::focus::notice::keys_notices;
use crate::protocol::notify_send::offer;
use crate::protocol::{encode_notify, NOTIFY_KIND_KEYS, NOTIFY_LEN};

/// Send each owner its own notice when focus moved from `was` to `now`. A
/// full inbox is offered the event again as a broadcast is; an owner that is
/// gone is simply not told.
pub fn tell_owners(was: Option<FocusedRef>, now: Option<FocusedRef>) {
    for (pid, window_id, has) in keys_notices(was, now).into_iter().flatten() {
        let mut frame = [0u8; NOTIFY_LEN];
        encode_notify(&mut frame, NOTIFY_KIND_KEYS, pid, window_id, has as u32, 0);
        let send = || mk_ipc_send_to_pid(pid, frame.as_ptr(), frame.len());
        let _ = offer(send, || {
            let _ = mk_yield();
        });
    }
}
