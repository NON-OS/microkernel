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

//! The kernel's boot census (MkBootCensus) as About shows it: one row a
//! device class, its state and, for one missing or held, why; the verdict
//! last. Read through nonos_hwreport's census parser, so About says only
//! what the kernel's lines say. Pure, proved in hwreport_proofs.

use nonos_hwreport::census::{parse, Census, State};

/// The classes and the verdict.
pub const ROWS: usize = 12;
/// A value's room: the state and its reason, cut to fit.
pub const VALUE: usize = 96;

fn label(class: &[u8]) -> &'static [u8] {
    match class {
        b"battery" => b"Battery",
        b"ac" => b"AC adapter",
        b"lid" => b"Lid",
        b"thermal" => b"Thermal zones",
        b"keyboard" => b"Keyboard",
        b"pointer" => b"Touchpad",
        b"display" => b"Display",
        b"storage" => b"Storage",
        b"network" => b"Network",
        b"usb" => b"USB",
        b"audio" => b"Audio",
        _ => b"Other",
    }
}

fn put(buf: &mut [u8; VALUE], at: usize, s: &[u8]) -> usize {
    let n = s.len().min(VALUE - at);
    buf[at..at + n].copy_from_slice(&s[..n]);
    at + n
}

/// One census line: its label, and its value written into `buf` (the
/// value's length); None for a line that is not the census's.
pub fn row(line: &[u8], buf: &mut [u8; VALUE]) -> Option<(&'static [u8], usize)> {
    match parse(line)? {
        Census::Row(r) => {
            let word: &[u8] = match r.state {
                State::Up => b"Up",
                State::Absent => b"None",
                State::Missing => b"Missing: ",
                State::Held => b"Present, not read: ",
            };
            let n = put(buf, 0, word);
            let n = match (r.state, r.why) {
                (State::Missing | State::Held, Some(why)) => put(buf, n, why),
                _ => n,
            };
            Some((label(r.class), n))
        }
        Census::Summary(s) => {
            let n = if s.green {
                put(buf, 0, b"Every device up")
            } else {
                let n = put(buf, 0, &[b'0' + s.missing.min(9)]);
                put(buf, n, if s.missing == 1 { b" device missing" } else { b" devices missing" })
            };
            Some((b"Census", n))
        }
    }
}
