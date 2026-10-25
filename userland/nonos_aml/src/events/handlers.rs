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

//! The GPE control methods under \_GPE (ACPI 6.5, 5.6.4.1): _Lxx runs for a
//! level-triggered event and _Exx for an edge-triggered one, xx the event's
//! number in hexadecimal.

use crate::namespace::{Id, Kind, Namespace, ROOT};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigger {
    Level,
    Edge,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Handler {
    pub method: Id,
    pub trigger: Trigger,
}

fn hex(c: u8) -> Option<u16> {
    match c {
        b'0'..=b'9' => Some(u16::from(c - b'0')),
        b'A'..=b'F' => Some(u16::from(c - b'A' + 10)),
        _ => None,
    }
}

/// The method for each event, and the events that have both an _Lxx and an
/// _Exx: firmware's error, kept to the one defined first.
pub fn handlers(ns: &Namespace) -> (BTreeMap<u16, Handler>, Vec<u16>) {
    let mut found = BTreeMap::new();
    let mut both = Vec::new();
    let Some(gpe) = ns.child(ROOT, *b"_GPE") else { return (found, both) };
    for id in ns.children(gpe) {
        let Some(e) = ns.get(id) else { continue };
        let trigger = match (&e.kind, e.seg[0], e.seg[1]) {
            (Kind::Method(_), b'_', b'L') => Trigger::Level,
            (Kind::Method(_), b'_', b'E') => Trigger::Edge,
            _ => continue,
        };
        let (Some(hi), Some(lo)) = (hex(e.seg[2]), hex(e.seg[3])) else { continue };
        let n = hi * 16 + lo;
        if found.contains_key(&n) {
            both.push(n);
            continue;
        }
        found.insert(n, Handler { method: id, trigger });
    }
    (found, both)
}
