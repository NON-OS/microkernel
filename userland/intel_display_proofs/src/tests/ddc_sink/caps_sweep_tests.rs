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

//! The capabilities read over GMBUS against seeded displays and bus
//! faults, stepped as the serve loop steps it. Each read is held to the
//! judge's reading of the sink's log: it settles exactly when the rule
//! ends, with the string only when the rule keeps it, and no reply read
//! past its 38 bytes' words.

use super::caps_judge::{judge_caps, End};
use super::caps_plan::{answer, plan};
use super::draw::fault;
use super::sink::{Fault, Sink};
use crate::ddc::pace::Pacer;
use crate::engine::gmbus_xfer::ddc::GmbusDdc;
use crate::model::Ticks;
use crate::tests::wire_sweep::rng::Rng;

#[test]
fn a_capabilities_read_over_gmbus_settles_as_the_rule_says() {
    let (mut r, mut seen) = (Rng(0x6900_CA95), std::collections::HashMap::new());
    for _ in 0..300 {
        let t = Ticks::new();
        let p = plan(&mut r);
        let mut s = Sink::new(&t, Vec::new());
        let mut nth = 0;
        s.ci = Some(Box::new(move |sent: &[u8]| {
            let offset = u16::from_be_bytes([sent[3], sent[4]]);
            nth += 1;
            answer(&p, offset, nth - 1)
        }));
        s.ci_nak = r.below(15) == 0;
        let rate = [0, 5, 30][r.below(3) as usize];
        s.faults = (0..5_400)
            .map(|_| if r.below(100) < rate { fault(&mut r) } else { Fault::None })
            .collect();
        let mut pace = Pacer::new();
        for _ in 0..5_400 {
            t.now.set(t.now.get() + 60_000);
            pace.fetch_caps(&mut GmbusDdc { mmio: &mut s, clock: &t, ddi: 1 }, t.now.get());
            if pace.caps.settled() {
                break;
            }
        }
        let end = judge_caps(&s.log);
        assert_eq!(pace.caps.settled(), end != End::Open, "settled when the rule ends");
        assert_eq!(pace.caps.caps().is_ok(), end == End::Got, "the string kept only whole");
        *seen.entry(end).or_insert(0) += 1;
    }
    for end in [End::Got, End::Absent, End::Over, End::Missed] {
        assert!(seen.get(&end).is_some_and(|&c| c >= 5), "{end:?} in {seen:?}");
    }
}
