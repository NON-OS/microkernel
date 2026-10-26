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
//! Only a request the server has dequeued can be answered: one still in
//! its inbox has no reply owed yet, and one already answered has no entry.
//! Of those, the request the server dequeued last names its call outright:
//! its entry wins whenever it is the replied-to caller's. Otherwise the
//! caller's oldest dequeued entry answers, as a server that takes several
//! requests and answers them in order needs. Never an undequeued entry: a
//! second reply to an answered request would take the caller's next call.

/// The position of the entry a reply to `inbox` answers among `entries`
/// (each reply inbox, token and whether its request was dequeued, oldest
/// first). `served` is the token the server dequeued last, 0 for none.
pub(crate) fn pick<'a, I>(mut entries: I, inbox: &str, served: u64) -> Option<usize>
where
    I: Iterator<Item = (&'a str, u64, bool)> + Clone,
{
    if served != 0 {
        let mut all = entries.clone();
        if let Some(at) = all.position(|(i, t, s)| s && t == served && i == inbox) {
            return Some(at);
        }
    }
    entries.position(|(i, _, s)| s && i == inbox)
}
