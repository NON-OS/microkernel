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

//! Every event enable cleared when the capsule leaves, and nothing else
//! written. The model names the enable bytes from the FADT's fields alone
//! (ACPI 6.5, 4.8.3.1 and 4.8.4.1, recalled): the second half of each PM1
//! event block 16 bits wide or more, and the second half of each GPE block
//! two bytes long or more, rounded down to whole registers; nothing on a
//! hardware-reduced platform.

use super::bus::Bus;
use super::disarm_gen::layout;
use super::fadt_decode::{FadtInfo, FLAG_HW_REDUCED_ACPI};
use super::gas::Gas;
use super::leave_events::disarm;
use std::collections::BTreeSet;

/// The enable bytes, as (memory, address).
fn enables(f: &FadtInfo) -> BTreeSet<(bool, u64)> {
    let mut out = BTreeSet::new();
    if f.flags & FLAG_HW_REDUCED_ACPI != 0 {
        return out;
    }
    let mut span = |g: &Gas, from: u64, n: u64| {
        if g.address != 0 {
            out.extend((from..from + n).map(|i| (g.space == 0, g.address + i)));
        }
    };
    for evt in [&f.pm1a_evt, &f.pm1b_evt] {
        let half = u64::from(evt.bit_width / 16);
        span(evt, half, if evt.bit_width >= 16 { half } else { 0 });
    }
    for (g, len) in [(&f.gpe0, f.gpe0_len), (&f.gpe1, f.gpe1_len)] {
        let regs = u64::from(len / 2);
        span(g, regs, regs);
    }
    out
}

#[test]
fn every_enable_is_cleared_and_nothing_else_written() {
    let (mut cleared, mut none) = (0, 0);
    for seed in 1..=4000u64 {
        let s = &mut seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let (f, mut bus) = (layout(s), Bus::default());
        disarm(&mut bus, &f);
        let want = enables(&f);
        let wrote: BTreeSet<_> = bus.written.iter().copied().collect();
        assert_eq!(wrote, want, "seed {seed}: {f:?}");
        assert!(want.iter().all(|(m, a)| bus.byte(*m, *a) == 0), "seed {seed}");
        *(if want.is_empty() { &mut none } else { &mut cleared }) += 1;
    }
    assert!(cleared >= 1000 && none >= 100, "{cleared} cleared, {none} with nothing to clear");
}
