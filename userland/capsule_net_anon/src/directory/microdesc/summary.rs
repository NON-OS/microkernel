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

//! The `p` line kept whole: its verb and its port ranges.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::lines::{arg, args};
use crate::directory::number::port;

/// Ranges kept from one line. Tor cuts a summary at a comma within 1000
/// characters (MAX_EXITPOLICY_SUMMARY_LEN), so a live line lists a few dozen;
/// past this many the rest are dropped, and no port they list is let out.
pub const RANGES_MAX: usize = 64;

/// The ports a `p` line names and what it says of them. The default is no
/// line at all, which lets nothing out, as Tor reads a missing summary.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Summary {
    /// The ranges are the ports refused, not the ports let out.
    pub reject: bool,
    /// Low and high port of each range, both inclusive, in the line's order.
    pub ranges: Vec<(u16, u16)>,
    /// The line listed more than RANGES_MAX ranges.
    pub cut: bool,
}

/// Read a `p` line's arguments, or `None` when it is malformed: a verb other
/// than accept or reject, no list, anything after the list, or an element
/// that is not a port or a range of ports from 1 to 65535 with its low end
/// first. Tor's parse_short_policy refuses the whole line for any of these.
pub fn summary(rest: &[u8]) -> Option<Summary> {
    let reject = match arg(rest, 0)? {
        b"accept" => false,
        b"reject" => true,
        _ => return None,
    };
    if args(rest).count() != 2 {
        return None;
    }
    let mut out = Summary { reject, ranges: Vec::new(), cut: false };
    for element in arg(rest, 1)?.split(|b| *b == b',') {
        let range = range(element)?;
        if out.ranges.len() < RANGES_MAX {
            out.ranges.push(range);
        } else {
            out.cut = true;
        }
    }
    out.ranges.shrink_to_fit();
    Some(out)
}

fn range(element: &[u8]) -> Option<(u16, u16)> {
    let (low, high) = match element.iter().position(|b| *b == b'-') {
        Some(at) => (port(&element[..at])?, port(&element[at + 1..])?),
        None => port(element).map(|one| (one, one))?,
    };
    (low <= high).then_some((low, high))
}
