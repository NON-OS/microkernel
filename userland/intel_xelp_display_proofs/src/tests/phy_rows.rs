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
//! Every DisplayPort and eDP level pair lands on its own row of Vol 12's
//! tables (pp. 393, 394), stated here as (Swing Sel, N Scalar, Cursor,
//! Post Cursor 1); and the edges the other tests do not reach.

use crate::model::regs::Regs;
use nonos_intel_xelp_display::phy::signal::{pick, Signal};
use nonos_intel_xelp_display::phy::swing::program;
use nonos_intel_xelp_display::regs::clka::Combo;

const PAIRS: [(u32, u32); 10] =
    [(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (3, 0)];

#[test]
fn every_pair_its_row() {
    let dp = [
        (0b1010, 0x32, 0x3F, 0x00),
        (0b1010, 0x4F, 0x37, 0x08),
        (0b1100, 0x71, 0x2F, 0x10),
        (0b0110, 0x7D, 0x2B, 0x14),
        (0b1010, 0x4C, 0x3F, 0x00),
        (0b1100, 0x73, 0x34, 0x0B),
        (0b0110, 0x7F, 0x2F, 0x10),
        (0b1100, 0x6C, 0x3C, 0x03),
        (0b0110, 0x7F, 0x35, 0x0A),
        (0b0110, 0x7F, 0x3F, 0x00),
    ];
    let edp = [0b0000, 0b1000, 0b0001, 0b1001, 0b1000, 0b0001, 0b1001, 0b0001, 0b1001, 0b1001];
    for (i, (vs, pe)) in PAIRS.iter().enumerate() {
        let (r, ..) = pick(Signal::Dp { mbps: 2700, lanes: 4, vs: *vs, pe: *pe }).unwrap();
        assert_eq!((r.sel, r.n_scalar, r.cursor, r.post1), dp[i], "DP {vs} {pe}");
        let (r, ..) = pick(Signal::Edp { mbps: 5400, lanes: 4, vs: *vs, pe: *pe }).unwrap();
        assert_eq!(r.sel, edp[i], "eDP {vs} {pe}");
    }
}

#[test]
fn hdmi_at_600_mhz_is_six_gbits_with_loadgen() {
    // 600 MHz is 6 Gbit/s, "<= 6 GHz" in Vol 12's loadgen table (p. 392).
    let mut m = Regs::new(Some(0));
    program(&mut m, Combo::A, Signal::Hdmi { row: 0, symbol_khz: 600_000 }).unwrap();
    assert_eq!([0, 1, 2, 3].map(|n| m.get(0x162890 + 0x100 * n) >> 31), [0, 1, 1, 1]);
}

#[test]
fn training_already_on_is_turned_off_before_the_swing() {
    let mut m = Regs::new(Some(0));
    m.set(0x162894, 1 << 31);
    program(&mut m, Combo::A, Signal::Dp { mbps: 1620, lanes: 1, vs: 0, pe: 0 }).unwrap();
    let first_dw5 = m.writes.iter().position(|w| w.0 == 0x162694).unwrap();
    let first_dw2 = m.writes.iter().position(|w| w.0 == 0x162688).unwrap();
    assert_eq!(m.writes[first_dw5].1 >> 31, 0);
    assert!(first_dw5 < first_dw2);
}
