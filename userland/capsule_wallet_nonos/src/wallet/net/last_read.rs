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

//! The last read of each network, as the screens say it: the block the host
//! had, which host, over which network, and how long ago. Only a read that
//! came whole is one, so what is shown is what the chain said, never what
//! the wallet expects. Pure, so wallet_proofs holds the wording.

use alloc::format;
use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LastRead {
    pub block: u64,
    pub host: &'static str,
    /// How the request went: "Nym" or "Anyone".
    pub route: &'static str,
    pub at_ms: i64,
}

/// One line for one network, at `now`.
pub fn read_line(read: Option<LastRead>, now: i64) -> String {
    let Some(r) = read else { return String::from("not read since this boot") };
    let s = now.saturating_sub(r.at_ms).max(0) / 1000;
    let ago = match s {
        0..=59 => format!("{s} s ago"),
        60..=3599 => format!("{} min ago", s / 60),
        _ => format!("{} h ago", s / 3600),
    };
    format!("block {} from {} over {}, {ago}", grouped(r.block), r.host, r.route)
}

/* 23456789 as 23,456,789. */
fn grouped(n: u64) -> String {
    let digits = format!("{n}");
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}
