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

//! Where a running download stands, as a tier's card says it.

use alloc::format;
use alloc::string::String;

use crate::path::duration;
use crate::size::size;
use crate::status_wire::{Status, ANYONE, ANYONE_WAIT, BROKEN, CHECKING, DIRECT, NYM, STARTING};

/// Where a running download stands, as the fetcher answered it.
pub fn progress_line(s: &Status) -> String {
    let (done, total) = (size(s.done), size(s.total));
    let via = match s.route {
        NYM => "through the Nym mixnet",
        ANYONE => "over Anyone",
        DIRECT => "over a direct connection",
        _ => "",
    };
    match s.stage {
        ANYONE_WAIT if s.try_n > 0 => {
            format!("Anyone is building its circuit, step {} of {}", s.try_n, s.tries)
        }
        ANYONE_WAIT => String::from("Anyone is starting; the download waits for it"),
        STARTING => {
            String::from("The download is starting: finding its route and the files' sizes")
        }
        CHECKING => format!("{total} here; checking its SHA-256 against the signed pin"),
        /*
         * Through Nym, an exit that went silent: net.socks5 rotates to the
         * next, and the fetcher tries again on a new connection.
         */
        BROKEN if s.try_n > 0 && s.route == NYM => format!(
            "Nym exit did not answer; trying the next one ({} of {}), from {done} of {total}",
            s.try_n, s.tries
        ),
        BROKEN if s.try_n > 0 => format!(
            "The connection dropped; trying again ({} of {}), from {done} of {total} {via}",
            s.try_n, s.tries
        ),
        BROKEN => format!("The connection dropped at {done} of {total}; it resumes from there"),
        _ if s.rate == 0 => format!("{done} of {total} {via}, measuring the rate"),
        _ => format!(
            "{done} of {total} {via}, {}/s, {} left",
            size(s.rate),
            duration(s.total.saturating_sub(s.done) / s.rate)
        ),
    }
}
