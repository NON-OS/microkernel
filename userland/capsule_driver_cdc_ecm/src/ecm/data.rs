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

//! The data interface's alternate 1 (CDC ECM 1.2, 3.3, as recalled,
//! unverified): exactly one bulk IN and one bulk OUT endpoint and nothing
//! else, declared and present, in one interface descriptor. Then the pipes
//! taken are the only ones the function moves frames on.

use nonos_usbnet::desc::{walk, Interface, ENDPOINT, INTERFACE};
use nonos_usbnet::Pipes;

/// The alternate that carries the pipes; alternate 0 carries none.
pub const DATA_ON: u8 = 1;
const BULK: u8 = 0x02;

#[derive(Default)]
struct Tally {
    seen: u8,
    declared: u8,
    ins: u8,
    outs: u8,
    other: u8,
}

pub fn bulk_alt(raw: &[u8], data: u8, ifs: &[Interface]) -> Option<Pipes> {
    let (mut t, mut inside) = (Tally::default(), false);
    for d in walk(raw) {
        if d[1] == INTERFACE && d.len() >= 9 {
            inside = d[2] == data && d[3] == DATA_ON;
            if inside {
                (t.seen, t.declared) = (t.seen.saturating_add(1), d[4]);
            }
        } else if inside && d[1] == ENDPOINT && d.len() >= 7 {
            let n = match (d[3] & 0x03 == BULK, d[2] & 0x80 != 0) {
                (true, true) => &mut t.ins,
                (true, false) => &mut t.outs,
                _ => &mut t.other,
            };
            *n = n.saturating_add(1);
        }
    }
    let one_pair = (t.seen, t.declared, t.ins, t.outs, t.other) == (1, 2, 1, 1, 0);
    let on = ifs.iter().find(|i| i.number == data && i.alt == DATA_ON)?;
    (one_pair && on.has_bulk_pair()).then_some(on.pipes)
}
