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

//! The line a person reads when a guest's stream could not open: the
//! network and a short reason, in the run's own window. It never names the
//! host or the port: a run's window is the person's own, but it can end up
//! in a screenshot. Each reason is a sentence this capsule or route link
//! wrote, never a byte a guest chose, and the whole line fits the 256 bytes
//! one write takes, so it is never split.

use alloc::format;
use alloc::string::String;

use super::super::guest_route::Net;

/// The most a reason may take of the line.
const REASON_MAX: usize = 180;

/// What the line calls a network.
pub fn word(net: Net) -> &'static str {
    match net {
        Net::Anyone => "Anyone",
        Net::Nym => "Nym",
        Net::Unread => "the default network",
    }
}

/// `[LINUX] net: <network> could not open a stream: <reason>`, one line.
pub fn line(net: Net, why: &str) -> String {
    let why = why.char_indices().nth(REASON_MAX).map_or(why, |(cut, _)| &why[..cut]);
    let why: String =
        why.chars().map(|c| if c.is_ascii_graphic() || c == ' ' { c } else { '?' }).collect();
    format!("[LINUX] net: {} could not open a stream: {why}\n", word(net))
}
