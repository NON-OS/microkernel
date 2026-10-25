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
//! What swing programming refuses, untouched, and every table row it can
//! reach, swept over all level pairs and rates.

use crate::model::regs::Regs;
use nonos_intel_xelp_display::phy::signal::{pick, Signal};
use nonos_intel_xelp_display::phy::swing::program;
use nonos_intel_xelp_display::regs::clka::Combo;

#[test]
fn refused_untouched() {
    let bad = [
        // DP above 2.7 Gbit/s: the two "DP HBR2" tables (pp. 393, 394).
        Signal::Dp { mbps: 5400, lanes: 4, vs: 0, pe: 0 },
        Signal::Dp { mbps: 8100, lanes: 4, vs: 0, pe: 0 },
        // eDP HBR3 needs the VBT (p. 395).
        Signal::Edp { mbps: 8100, lanes: 4, vs: 0, pe: 0 },
        // An unlisted rate, levels past the table, a three-lane link.
        Signal::Dp { mbps: 2000, lanes: 4, vs: 0, pe: 0 },
        Signal::Dp { mbps: 2700, lanes: 4, vs: 2, pe: 2 },
        Signal::Edp { mbps: 2700, lanes: 4, vs: 4, pe: 0 },
        Signal::Dp { mbps: 2700, lanes: 3, vs: 0, pe: 0 },
        Signal::Hdmi { row: 7, symbol_khz: 148_500 },
        // HDMI outside 20 to 600 MHz (p. 170).
        Signal::Hdmi { row: 0, symbol_khz: 19_999 },
        Signal::Hdmi { row: 0, symbol_khz: 600_001 },
    ];
    for sig in bad {
        let mut m = Regs::new(Some(0));
        assert!(program(&mut m, Combo::A, sig).is_err(), "{sig:?}");
        assert!(m.writes.is_empty(), "{sig:?}");
    }
}

#[test]
fn every_dp_and_edp_level_pair() {
    // The pairs whose sum is at most 3 have rows; the rest do not (p. 393).
    for vs in 0..6 {
        for pe in 0..6 {
            let ok = vs + pe <= 3;
            for sig in [
                Signal::Dp { mbps: 1620, lanes: 1, vs, pe },
                Signal::Edp { mbps: 4320, lanes: 2, vs, pe },
            ] {
                assert_eq!(pick(sig).is_ok(), ok, "{sig:?}");
            }
        }
    }
}

#[test]
fn rates_offered() {
    let dp = |mbps| pick(Signal::Dp { mbps, lanes: 4, vs: 0, pe: 0 }).is_ok();
    let edp = |mbps| pick(Signal::Edp { mbps, lanes: 4, vs: 0, pe: 0 }).is_ok();
    let rates = [1620, 2160, 2700, 3240, 4320, 5400, 6480, 8100];
    assert_eq!(rates.map(dp), [true, true, true, false, false, false, false, false]);
    assert_eq!(rates.map(edp), [true, true, true, true, true, true, false, false]);
}
