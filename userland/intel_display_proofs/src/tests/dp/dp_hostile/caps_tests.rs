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

//! DPCD capabilities that lie: a revision of anything, a rate code the
//! standard does not define, a lane count of 0, 3 or past 4, an interval
//! code past 4. Each such read is no usable sink; every other read gives
//! a sink whose rate and width a source can program.

use super::rng::Rng;
use crate::dp::caps::Caps;
use crate::dp::dpcd::rate_mbps;

#[test]
fn a_capability_read_is_a_programmable_sink_or_none() {
    let mut r = Rng::new(0xD9C0_CA95);
    let mut seen = [0usize; 4];
    for _ in 0..200_000 {
        let mut b = [0u8; 16];
        b.iter_mut().for_each(|v| *v = r.byte());
        b[1] = [0x06, 0x0A, 0x14, 0x1E, r.byte()][r.below(5) as usize];
        b[2] = (b[2] & 0xE0) | [0, 1, 2, 3, 4, 5, r.byte() & 0x1F][r.below(7) as usize];
        b[0x0E] = (b[0x0E] & 0x80) | [0, 1, 2, 3, 4, 5, r.byte() & 0x7F][r.below(7) as usize];
        let rate = rate_mbps(b[1]);
        let lanes = (b[2] & 0x1F) as u32;
        let interval = b[0x0E] & 0x7F;
        let fit = rate.is_some() && matches!(lanes, 1 | 2 | 4) && interval <= 4;
        let got = Caps::decode(&b);
        assert_eq!(got.is_some(), fit, "{b:02X?}");
        if let Some(c) = got {
            assert_eq!((Some(c.max_rate_mbps), c.max_lanes, c.revision), (rate, lanes, b[0]));
            assert!(c.eq_interval_us == 400 || c.eq_interval_us == interval as u32 * 4000);
        }
        let k = match (rate.is_some(), matches!(lanes, 1 | 2 | 4), interval <= 4) {
            (true, true, true) => 0,
            (false, _, _) => 1,
            (_, false, _) => 2,
            _ => 3,
        };
        seen[k] += 1;
    }
    // [taken, bad rate, bad lanes, bad interval]
    assert!(seen.iter().all(|&n| n >= 10_000), "{seen:?}");
}
