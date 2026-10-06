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

//! Whether to ask the time server, decided before every exchange.
//!
//! SNTP is a UDP packet from this machine's address to a fixed server, which
//! no anonymity network carries, so asking it names this machine to the
//! server and to everyone between, at every boot. It was asked whatever
//! network the person had chosen. Now it is asked only when the default
//! network is Direct, by nonos_route_link's rule: an anonymous network, or a
//! policy store that cannot be read (the default, the Nym mixnet), keeps the
//! clock the RTC gave and says so once.

use alloc::format;
use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// Ask the time server.
    Sync,
    /// Keep the RTC time. `log` is whether `why` is news since the last look.
    Skip { why: &'static str, log: bool },
}

/// The next step, from the route's reason to refuse a direct contact (None
/// when Direct is chosen) and the reason last logged.
pub fn step(refused: Option<&'static str>, said: Option<&'static str>) -> Step {
    match refused {
        None => Step::Sync,
        Some(why) => Step::Skip { why, log: said != Some(why) },
    }
}

/// The one serial line a skip prints.
pub fn skip_line(why: &str) -> String {
    format!(
        "[NTP] time sync skipped: {why}; a time server would see this machine, \
         so the RTC time is kept\n"
    )
}
