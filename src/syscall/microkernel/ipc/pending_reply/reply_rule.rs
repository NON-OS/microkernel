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

//! Which pending entry a direct `mk_ipc_reply` answers. Pure, so the host
//! runs it (kernel_proofs::ipc_reply_token).
//!
//! The request the server dequeued last names its call outright: its entry
//! wins whenever it is the replied-to caller's. Only when the server has
//! dequeued someone else's request since does the caller's oldest entry
//! answer, as a server pipelining requests and replying in order needs.
//! Never the oldest by default: a timed-out call keeps its entry
//! (call/settle.rs), so a request the server never answered would stamp
//! every later reply to that caller with the call before it.

/// The position of the entry a reply to `inbox` answers among `entries`
/// (each reply inbox and token, oldest first). `served` is the token the
/// server dequeued last, 0 for none.
pub(crate) fn pick<'a, I>(mut entries: I, inbox: &str, served: u64) -> Option<usize>
where
    I: Iterator<Item = (&'a str, u64)> + Clone,
{
    if served != 0 {
        let mut all = entries.clone();
        if let Some(at) = all.position(|(i, t)| t == served && i == inbox) {
            return Some(at);
        }
    }
    entries.position(|(i, _)| i == inbox)
}
