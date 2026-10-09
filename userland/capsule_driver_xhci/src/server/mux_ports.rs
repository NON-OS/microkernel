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

//! Root port numbers across several controllers: the primary's from 1, each
//! next controller's after the last of the one before, none past 255 (the
//! wire carries a port in one byte).

use alloc::vec::Vec;

const MAX_GLOBAL_PORT: u16 = 255;

/// Global port numbers for controllers with `max_ports` each: consecutive,
/// the first from 1, cut off at 255.
pub fn port_ranges(max_ports: &[u8]) -> Vec<(u8, u8)> {
    let mut base = 0u16;
    max_ports
        .iter()
        .map(|&n| {
            let count = (n as u16).min(MAX_GLOBAL_PORT - base);
            let range = (base as u8, count as u8);
            base += count;
            range
        })
        .collect()
}

/// The controller and its own port number for global port `port`.
pub fn local_port(ranges: &[(u8, u8)], port: u8) -> Option<(usize, u8)> {
    ranges.iter().enumerate().find_map(|(i, &(base, count))| {
        (port > base && port <= base.saturating_add(count)).then(|| (i, port - base))
    })
}
