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
//! the project's own window, so no app learns which other app is in front.

use nonos_libc::{mk_ipc_send_to_pid, mk_pid_alive, mk_yield};

use crate::focus::model::FocusedRef;
use crate::focus::notice::keys_notices;
use crate::focus::owed_keys::Owed;
use crate::protocol::notify_send::{sent, Sent, BUSY_TRIES};
use crate::protocol::{encode_notify, NOTIFY_KIND_KEYS, NOTIFY_LEN};
use crate::state::Context;

/// Tell the project's of the window that lost the keyboard and the one that took
/// it. A notice the project's full inbox does not take within a few yields is
/// owed, and offered again each turn (resend_owed), so a window never misses
/// the word that it has the keyboard.
pub fn tell_owners(ctx: &mut Context, was: Option<FocusedRef>, now: Option<FocusedRef>) {
    for (owner_pid, window_id, has) in keys_notices(was, now).into_iter().flatten() {
        let notice = Owed { owner_pid, window_id, has };
        ctx.owed_keys.tell(
            notice,
            BUSY_TRIES,
            |n| send(n) == Sent::Again,
            || {
                let _ = mk_yield();
            },
        );
    }
}

/// Offer each owed notice once more; an owner that has exited is owed none.
pub fn resend_owed(ctx: &mut Context) {
    ctx.owed_keys.offer_again(|owed| mk_pid_alive(owed.owner_pid) && send(owed) == Sent::Again);
}

fn send(notice: &Owed) -> Sent {
    let mut frame = [0u8; NOTIFY_LEN];
    let has = notice.has as u32;
    encode_notify(&mut frame, NOTIFY_KIND_KEYS, notice.owner_pid, notice.window_id, has, 0);
    sent(mk_ipc_send_to_pid(notice.owner_pid, frame.as_ptr(), frame.len()))
}
