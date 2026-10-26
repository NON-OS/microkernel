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

//! One call owed a reply: who made it, the inbox its reply goes to, its
//! token, the id the server knows it by, and whether the server has
//! dequeued its request. A reply may answer only a request the server has
//! taken (reply_rule.rs, served_rule.rs).

use alloc::string::String;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Entry {
    pub(crate) pid: u32,
    pub(crate) inbox: String,
    pub(crate) token: u64,
    /// The server's name for the request (request_ids.rs), never 0.
    pub(crate) request: u64,
    /// The server's receive took this request (mark.rs).
    pub(crate) served: bool,
}

impl Entry {
    pub(crate) fn new(pid: u32, inbox: String, token: u64, request: u64) -> Self {
        Entry { pid, inbox, token, request, served: false }
    }
}
