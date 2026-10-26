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

//! Which pending entry a dequeue marks, and which one a redirect reply
//! answers. Pure, so the host runs both (kernel_proofs::ipc_reply_token).

use super::entry::Entry;

/// Mark the call carrying `token` as dequeued: from here a reply may
/// answer it. A token no entry carries marks nothing.
pub(crate) fn mark<'a, I>(mut queue: I, token: u64)
where
    I: Iterator<Item = &'a mut Entry>,
{
    if let Some(e) = queue.find(|e| e.token == token) {
        e.served = true;
    }
}

/// The position of the entry a redirect reply answers: the call carrying
/// `want`, the token the server dequeued last, so dequeued by definition.
/// None for 0, a request no capsule made (a kernel sender's), or for one
/// already answered.
pub(crate) fn redirect_at<'a, I>(mut queue: I, want: u64) -> Option<usize>
where
    I: Iterator<Item = &'a Entry>,
{
    if want == 0 {
        return None;
    }
    queue.position(|e| e.token == want)
}
