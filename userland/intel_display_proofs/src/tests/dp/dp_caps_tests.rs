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

//! The receiver capability fields.

use crate::dp::caps::Caps;
use crate::model::sink::sink;
use crate::model::sink_aux::AuxEnd;

fn caps(b: [u8; 4], interval: u8) -> Option<Caps> {
    let mut d = [0u8; 16];
    d[..4].copy_from_slice(&b);
    d[0x0E] = interval;
    Caps::decode(&d)
}

#[test]
fn a_dp_1_4_sink_reads_back_whole() {
    let c = Caps::read(&mut AuxEnd(sink(vec![]))).unwrap().unwrap();
    assert_eq!((c.revision, c.max_rate_mbps, c.max_lanes), (0x14, 8100, 4));
    assert!(c.tps3 && c.enhanced_framing && !c.tps4);
    assert_eq!(c.eq_interval_us, 400);
}

#[test]
fn each_rate_code_and_the_interval_steps() {
    assert_eq!(caps([0x12, 0x0A, 2, 0], 0).unwrap().max_rate_mbps, 2700);
    assert_eq!(caps([0x11, 0x06, 1, 0], 3).unwrap().eq_interval_us, 12_000);
    assert_eq!(caps([0x14, 0x14, 4, 0x80], 4).unwrap().eq_interval_us, 16_000);
    assert!(caps([0x14, 0x14, 4, 0x80], 4).unwrap().tps4);
}

#[test]
fn an_undefined_rate_lane_count_or_interval_is_no_usable_sink() {
    assert_eq!(caps([0x12, 0x0B, 4, 0], 0), None);
    assert_eq!(caps([0x12, 0x0A, 3, 0], 0), None);
    assert_eq!(caps([0x12, 0x0A, 0, 0], 0), None);
    assert_eq!(caps([0x12, 0x0A, 4, 0], 5), None);
}
