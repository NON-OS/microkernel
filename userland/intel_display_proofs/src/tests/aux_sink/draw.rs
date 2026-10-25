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

//! A DisplayPort sink's lies drawn from the seed: G-69's lies about the
//! pointer, the index and one try's bytes, and G-66's kinds of AUX answer
//! at rates from none to every transfer.

use super::sink::{AuxSink, Rates};
use crate::tests::ddc_sink::draw::{fault, SLOW_MAX};
use crate::tests::ddc_sink::sink::{Eddc, Fault};
use crate::tests::wire_sweep::rng::Rng;

/// The most one transfer may take: the reply timer and a slow try's hold.
pub const TRANSFER_MAX: u64 = 1_600 + SLOW_MAX + 1_000;

pub fn lie(s: &mut AuxSink, r: &mut Rng) {
    s.eddc = match r.below(6) {
        0 => Eddc::Nak,
        1 => Eddc::Ignore,
        2 => Eddc::Off([1, 2, 255][r.below(3) as usize]),
        _ => Eddc::Honest,
    };
    s.ignore_index = r.below(10) == 0;
    let rare = |r: &mut Rng| [0, 0, 0, 1, 3, 20, 100][r.below(7) as usize];
    let mut p = Rates {
        timeout: rare(r),
        defer: rare(r),
        nack: rare(r) / 4,
        undefined: rare(r) / 8,
        empty: rare(r) / 4,
        odd: [0, 2, 4, 16][r.below(4) as usize],
    };
    if p.timeout + p.defer + p.nack + p.undefined + p.empty > 100 {
        p = Rates { timeout: 50, defer: 50, ..p };
    }
    s.rates = p;
    s.seed = r.next() | 1;
    let rate = [0, 25, 50, 90][r.below(4) as usize];
    s.faults = (0..12).map(|_| if r.below(100) < rate { fault(r) } else { Fault::None }).collect();
}
