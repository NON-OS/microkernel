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

//! A sink's lies, drawn from the seed: its segment pointer refused,
//! ignored or off; its index ignored; now and then not on the pins at all;
//! and for each try, at one of four rates, one fault in its read; or a
//! staller, whose tries mostly hold the bus until their deadline.

use super::sink::{Eddc, Fault, Sink};
use crate::tests::wire_sweep::rng::Rng;

/// Each word held this long at most: past one try's deadline in one word.
pub const SLOW_MAX: u64 = 120_000;
const SLOW: [u64; 4] = [100, 3_000, 4_000, SLOW_MAX];

pub fn lie(s: &mut Sink, r: &mut Rng) {
    s.eddc = match r.below(6) {
        0 => Eddc::Nak,
        1 => Eddc::Ignore,
        2 => Eddc::Off([1, 2, 255][r.below(3) as usize]),
        _ => Eddc::Honest,
    };
    s.ignore_index = r.below(10) == 0;
    s.pins = if r.below(40) == 0 { 4 } else { 5 };
    let rate = [0, 25, 50, 90, 70][r.below(5) as usize];
    let pick = |r: &mut Rng| match rate {
        70 => [Fault::Stall(0), Fault::NoPhase][r.below(2) as usize],
        _ => fault(r),
    };
    s.faults = (0..12).map(|_| if r.below(100) < rate { pick(r) } else { Fault::None }).collect();
}

pub fn fault(r: &mut Rng) -> Fault {
    match r.below(9) {
        0 => Fault::Nak(r.below(34) as usize),
        1 => Fault::Stall(r.below(34) as usize),
        2 => Fault::Drop(r.below(128) as usize),
        3 => Fault::Again(r.below(32) as usize),
        4 => Fault::Index(r.below(256) as u8),
        5 => Fault::Slow(SLOW[r.below(4) as usize]),
        6 => Fault::NoPhase,
        7 => Fault::Over,
        _ => Fault::None,
    }
}
