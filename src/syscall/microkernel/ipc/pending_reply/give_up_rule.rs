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

//! A call that stops waiting, and the MkIpcReplyTo that comes for it, in
//! either order (call/given_up.rs, take_named.rs). Pure, so the host runs
//! every order (kernel_proofs::ipc_reply_token): an MkIpcReplyTo returns 0
//! exactly when its caller has the answer.

use alloc::collections::VecDeque;

use super::entry::Entry;

/// What the caller's give-up found of its entry.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GiveUp {
    /// Owed still: now given up, and an answer will return ETIMEDOUT.
    Marked,
    /// An answer is being delivered: wait for it to land, then take it.
    Delivering,
    /// Answered, or the server is gone: whatever came is in the inbox.
    Gone,
}

/// The caller of `token` stops waiting.
pub(crate) fn mark_given_up<'a, I>(mut queue: I, token: u64) -> GiveUp
where
    I: Iterator<Item = &'a mut Entry>,
{
    match queue.find(|e| e.token == token) {
        None => GiveUp::Gone,
        Some(e) if e.taken => GiveUp::Delivering,
        Some(e) => {
            e.given_up = true;
            GiveUp::Marked
        }
    }
}

/// An MkIpcReplyTo found `e`: true when it is owed and now taken for
/// delivery; false when its call gave up, and the answer is spent unsent.
pub(crate) fn take(e: &mut Entry) -> bool {
    if e.given_up {
        return false;
    }
    e.taken = true;
    true
}

/// The answer to `request` was delivered, or could not be: its taken
/// entry leaves the queue.
pub(crate) fn done(queue: &mut VecDeque<Entry>, request: u64) {
    queue.retain(|e| !(e.taken && e.request == request));
}
