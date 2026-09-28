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

//! The `p` line: whether a relay will carry a stream to a given port.

use crate::directory::lines::arg;
use crate::directory::number::decimal;

const WEB_PORTS: [u64; 2] = [80, 443];

/// Whether the summary lets every web port out.
///
pub fn exits_web(rest: &[u8]) -> bool {
    let Some(verb) = arg(rest, 0) else { return false };
    let Some(ranges) = arg(rest, 1) else { return false };
    let listed = WEB_PORTS.iter().all(|port| in_ranges(ranges, *port));
    match verb {
        b"accept" => listed,
        b"reject" => WEB_PORTS.iter().all(|port| !in_ranges(ranges, *port)),
        _ => false,
    }
}

fn in_ranges(ranges: &[u8], port: u64) -> bool {
    for element in ranges.split(|b| *b == b',') {
        match element.iter().position(|b| *b == b'-') {
            Some(at) => {
                let low = decimal(&element[..at]);
                let high = element.get(at + 1..).and_then(decimal);
                if let (Some(low), Some(high)) = (low, high) {
                    if port >= low && port <= high {
                        return true;
                    }
                }
            }
            None => {
                if decimal(element) == Some(port) {
                    return true;
                }
            }
        }
    }
    false
}
