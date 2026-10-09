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

//! The SSC settings rtsx_pci_switch_clock computes, worked by hand from its
//! code for the two clocks the driver uses, and the clock budget.

use crate::budget::Budget;
use crate::regs::clk::{SSC_DEPTH_4M, SSC_DEPTH_500K};
use crate::regs::sd::{SD_CLK_DIVIDE_0, SD_CLK_DIVIDE_128};
use crate::wire::clock_plan::{plan, ClockPlan};

#[test]
fn identification_runs_30_mhz_through_the_128_divider() {
    // clk 30: n 28, doubled twice to 118 with div 3; mcu_cnt 125/30+3 = 7;
    // depth 500K (4) doubled to 3, revised by div - 1 = 2 to 1.
    let want = ClockPlan {
        divider: SD_CLK_DIVIDE_128,
        clk_mhz: 30,
        n: 118,
        div: 3,
        mcu_cnt: 7,
        ssc_depth: 1,
    };
    assert_eq!(plan(400_000, SSC_DEPTH_500K, true, true), Some(want));
}

#[test]
fn default_speed_is_25_mhz_from_a_doubled_50_mhz_ssc() {
    // clk 50: n 48, doubled once to 98 with div 2; mcu_cnt 125/50+3 = 5;
    // depth 4 doubled to 3, revised by 1 to 2.
    let want = ClockPlan {
        divider: SD_CLK_DIVIDE_0,
        clk_mhz: 50,
        n: 98,
        div: 2,
        mcu_cnt: 5,
        ssc_depth: 2,
    };
    assert_eq!(plan(25_000_000, SSC_DEPTH_500K, false, true), Some(want));
}

#[test]
fn a_depth_the_divider_eats_falls_back_to_4m_and_the_range_is_kept() {
    let p = plan(25_000_000, SSC_DEPTH_4M, false, true).unwrap();
    assert_eq!(p.ssc_depth, SSC_DEPTH_4M);
    // 208 MHz doubled needs n = 414, past MAX_DIV_N_PCR.
    assert_eq!(plan(208_000_000, SSC_DEPTH_500K, false, true), None);
    assert_eq!(plan(1_000_000, SSC_DEPTH_500K, false, false), None);
}

#[test]
fn a_failed_clock_read_spends_the_budget() {
    let b = Budget::begin(Some(1000), 100);
    assert!(!b.spent(Some(1099)));
    assert!(b.spent(Some(1100)));
    assert!(b.spent(None));
    assert!(Budget::begin(None, 100).spent(Some(0)));
}
