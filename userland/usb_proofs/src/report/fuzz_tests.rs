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

//! Two hundred thousand hostile report descriptors through the walk, and
//! hostile reports through every layout it takes. Nothing may panic; every
//! field of a bindable layout lies inside its longest report; nothing read
//! through a layout leaves the defined key, button and position ranges.

use super::fuzz_reads::{check_reads, ends};
use super::hostile_gen::hostile_descriptor;
use crate::descriptors::report::parse;
use crate::fuzz::ROUNDS;
use crate::rng::Rng;

#[test]
fn hostile_report_descriptors_never_panic_and_bind_only_what_fits() {
    let mut rng = Rng::seeded(0x5eed_0006);
    for round in 0..ROUNDS {
        let Ok(l) = parse(&hostile_descriptor(&mut rng)) else { continue };
        assert!(l.longest <= 257, "round {round}: {l:?}");
        let body_bits = u32::from(l.longest.saturating_sub(u16::from(l.ids))) * 8;
        if l.bindable(8) {
            assert!(ends(&l).iter().all(|&e| e <= body_bits), "round {round}: {l:?}");
        }
        check_reads(&l, &mut rng, round);
    }
}
