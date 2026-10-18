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

//! The EDID read swept against seeded hostile sinks. Each read is held to
//! the judge's reading of the sink's log: only whole blocks kept, exactly
//! those the rule keeps; nothing written past them; no more than BLOCKS
//! times TRIES tries; never past the read's deadline, and short of it
//! only when the rule is done.

use super::draw::{lie, SLOW_MAX};
use super::edid_gen::edid;
use super::judge::{judge, BLOCKS, TRIES};
use super::kit::{EDID_US, HDMI_B};
use super::sink::Sink;
use crate::display::{read_edid, EDID_MAX};
use crate::engine::gmbus_xfer::deadline::XFER_US;
use crate::model::Ticks;
use crate::tests::wire_sweep::rng::Rng;

#[test]
fn a_hostile_sink_gets_only_whole_blocks_and_within_the_bounds() {
    let (mut r, mut lens, mut retried, mut cut) = (Rng(0x6900_D1CE), [0usize; 5], 0, 0);
    for _ in 0..3_000 {
        let t = Ticks::new();
        let mut s = Sink::new(&t, edid(&mut r));
        lie(&mut s, &mut r);
        let mut out = [0xA5u8; EDID_MAX];
        let start = t.now.get();
        let n = read_edid(&mut s, &t, &HDMI_B, &mut out);
        let whole_end = start + 50 + EDID_US;
        let v = judge(&s.log, whole_end, XFER_US);
        assert_eq!(&out[..n], &v.kept[..], "what the rule keeps");
        assert!(out[n..].iter().all(|&b| b == 0xA5), "nothing past the blocks kept");
        assert!(s.log.len() <= BLOCKS * TRIES);
        assert!(t.now.get() <= whole_end + SLOW_MAX + 1_000, "past the read's deadline");
        assert!(!v.cut || t.now.get() >= whole_end, "gave up before its deadline");
        (lens[n / 128], retried, cut) =
            (lens[n / 128] + 1, retried + v.retried as u32, cut + v.cut as u32);
    }
    assert!(lens.iter().all(|&c| c >= 100), "every length {lens:?}");
    assert!(retried >= 100 && cut >= 30, "retried {retried}, cut {cut}");
}
