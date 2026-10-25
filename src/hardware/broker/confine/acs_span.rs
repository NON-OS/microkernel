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

//! The groups below switches with a port the ACS controls leave open (L72):
//! everything below such a switch may be reached from anything else below
//! it past the IOMMU. Pure, so the grouping runs on the host too.

extern crate alloc;

use alloc::vec::Vec;

/// One switch downstream port: the bus it sits on, which is its switch's,
/// the buses below it, and whether it is closed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Port {
    pub bus: u8,
    pub below: Option<(u8, u8)>,
    pub closed: bool,
}

/// The groups, as `(first, last)` bus: the buses below each switch with an
/// open port, spans that share a bus merged. A port whose range cannot be
/// read is taken to reach every bus after its own.
pub(super) fn groups(ports: &[Port]) -> Vec<(u8, u8)> {
    let mut spans: Vec<(u8, u8)> = Vec::new();
    for open in ports.iter().filter(|p| !p.closed && p.bus < u8::MAX) {
        let reach = |p: &Port| match p.below {
            Some((first, last)) if first > p.bus && last >= first => last,
            _ => u8::MAX,
        };
        let last = ports.iter().filter(|p| p.bus == open.bus).map(reach).max();
        spans.push((open.bus + 1, last.unwrap_or(u8::MAX)));
    }
    spans.sort_unstable();
    let mut merged: Vec<(u8, u8)> = Vec::new();
    for (first, last) in spans {
        match merged.last_mut() {
            Some(m) if first <= m.1 => m.1 = m.1.max(last),
            _ => merged.push((first, last)),
        }
    }
    merged
}

/// The group a device on `bus` belongs to, by its first bus.
pub(super) fn group_of(groups: &[(u8, u8)], bus: u8) -> Option<u8> {
    groups.iter().find(|g| g.0 <= bus && bus <= g.1).map(|g| g.0)
}
