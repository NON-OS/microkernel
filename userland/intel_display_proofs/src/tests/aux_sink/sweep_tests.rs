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

//! The EDID read over AUX swept against seeded hostile DisplayPort sinks,
//! held as the GMBUS read is (`ddc_sink/edid_sweep_tests.rs`): exactly the
//! judge's keep, nothing written past it, at most BLOCKS x TRIES tries,
//! never past the read's deadline but for the transfers already under way
//! and the stop's own retries, and short of it only when the rule is done.

use super::draw::{lie, TRANSFER_MAX};
use super::judge::judge;
use super::regs::AuxRegs;
use super::sink::AuxSink;
use crate::display::{read_edid, EDID_MAX};
use crate::dp::native::RETRIES;
use crate::engine::route::{Link, Route};
use crate::model::Ticks;
use crate::tests::ddc_sink::edid_gen::edid;
use crate::tests::ddc_sink::judge::{BLOCKS, TRIES};
use crate::tests::ddc_sink::kit::EDID_US;
use crate::tests::wire_sweep::rng::Rng;

pub const DP_B: Route = Route { ddi: 1, link: Link::Dp { lanes: 4 } };

#[test]
fn a_hostile_dp_sink_gets_only_whole_blocks_through_e_ddc_within_the_bounds() {
    let (mut r, mut lens, mut retried, mut cut) = (Rng(0x7100_A0C5), [0usize; 5], 0, 0);
    for _ in 0..5_000 {
        let t = Ticks::new();
        let mut s = AuxSink::new(edid(&mut r));
        lie(&mut s, &mut r);
        let mut regs = AuxRegs::new(&t, s);
        let mut out = [0xA5u8; EDID_MAX];
        let start = t.now.get();
        let n = read_edid(&mut regs, &t, &DP_B, &mut out);
        regs.sink.close(t.now.get());
        let whole_end = start + 50 + EDID_US;
        let v = judge(&regs.sink.log);
        assert_eq!(&out[..n], &v.kept[..], "what the rule keeps");
        assert!(out[n..].iter().all(|&b| b == 0xA5), "nothing past the blocks kept");
        assert!(regs.sink.log.len() <= BLOCKS * TRIES);
        let over = (RETRIES as u64 + 2) * TRANSFER_MAX;
        assert!(t.now.get() <= whole_end + over, "past the read's deadline");
        assert!(!v.cut || t.now.get() >= whole_end, "gave up before its deadline");
        (lens[n / 128], retried, cut) =
            (lens[n / 128] + 1, retried + v.retried as u32, cut + v.cut as u32);
    }
    assert!(lens.iter().all(|&c| c >= 100), "every length {lens:?}");
    assert!(retried >= 100 && cut >= 10, "retried {retried}, cut {cut}");
}
