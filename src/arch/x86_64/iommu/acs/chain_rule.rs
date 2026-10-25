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

//! The chain above a bridge, read from the root down (L77c): which bridges
//! end a link where they sit, and which buses only upstream ports so borne
//! out name. Apart from `port_rule` for its length; pure, for the host too.

use super::port_rule::{Bridge, DOWNSTREAM, ROOT_PORT, UPSTREAM};

/// The bridges naming `bus` as their secondary, each from a bus above it.
pub(super) fn naming(bridges: &[Bridge], bus: u8) -> impl Iterator<Item = &Bridge> {
    bridges.iter().filter(move |b| b.1 == Some(bus) && bus > b.0)
}

/// Whether `b` ends a link where it sits: a root port nothing names, or a
/// downstream port every bridge above which is a borne-out upstream port;
/// and either way one whose bus below holds one bridge at most, since a link
/// carries one component and two bridges there are a switch's ports. A
/// switch on a root bus with no root port above it, its upstream port saying
/// root, is found by that alone (L77d).
pub(super) fn ends(bridges: &[Bridge], ups: &[bool; 256], b: &Bridge) -> bool {
    let on = |s: u8| bridges.iter().filter(|x| x.0 == s).count();
    if b.1.is_some_and(|s| on(s) > 1) {
        return false;
    }
    match b.2 {
        Some(ROOT_PORT) => naming(bridges, b.0).next().is_none(),
        Some(DOWNSTREAM) => ups[b.0 as usize],
        _ => false,
    }
}

/// Per bus, whether bridges name it and each says upstream below bridges
/// that all end links. Buses are taken upward, each asking only buses above
/// it already settled, so a list naming one bus many times costs no more.
pub(super) fn chain(bridges: &[Bridge]) -> [bool; 256] {
    let (mut ups, mut ended) = ([false; 256], [false; 256]);
    for bus in 0..=u8::MAX {
        let named = naming(bridges, bus).next().is_some();
        ended[bus as usize] = named && naming(bridges, bus).all(|b| ends(bridges, &ups, b));
        let up = |a: &Bridge| a.2 == Some(UPSTREAM) && ended[a.0 as usize];
        ups[bus as usize] = named && naming(bridges, bus).all(up);
    }
    ups
}
