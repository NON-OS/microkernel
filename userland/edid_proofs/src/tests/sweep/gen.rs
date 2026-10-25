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

//! Descriptor slots and DisplayID records of every kind a monitor might
//! send, each marked with what it is so a sweep knows what to expect.

use super::lie::{tell, ALL};
use super::plan::{plan, Plan};
use super::rng::Rng;
use super::widths::{Widths, DTD, TYPE7};
use super::write::{as_dtd, as_record};
use crate::build::display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Honest,
    Lied,
    Display,
    Noise,
    /// All zero: a display descriptor with tag 0, or a CTA-861 end.
    Blank,
}

fn lied(r: &mut Rng, w: &Widths) -> Plan {
    loop {
        let mut p = plan(r, w);
        if tell(&mut p, ALL[r.range(0, 7) as usize], w) {
            return p;
        }
    }
}

pub fn slot(r: &mut Rng) -> ([u8; 18], Kind) {
    let tags = [0xFC, 0xFD, 0xFE, 0xFF, 0xFA, 0xF8, 0xF7, 0x10, r.byte()];
    match r.range(0, 4) {
        0 => (as_dtd(&plan(r, &DTD), r), Kind::Honest),
        1 => (as_dtd(&lied(r, &DTD), r), Kind::Lied),
        2 => (display(tags[r.range(0, 8) as usize], &r.bytes::<13>()), Kind::Display),
        3 => {
            // A zero clock would make it a display descriptor.
            let mut d = r.bytes::<18>();
            d[0] |= 1;
            (d, Kind::Noise)
        }
        _ => ([0; 18], Kind::Blank),
    }
}

/// A record counting its clock in `unit_khz`.
pub fn record(r: &mut Rng, unit_khz: u32) -> ([u8; 20], Kind) {
    let w = Widths { clock_step_khz: unit_khz, clock_max_khz: unit_khz << 24, ..TYPE7 };
    match r.range(0, 2) {
        0 => (as_record(&plan(r, &w), unit_khz, r), Kind::Honest),
        1 => (as_record(&lied(r, &w), unit_khz, r), Kind::Lied),
        _ => (r.bytes(), Kind::Noise),
    }
}
