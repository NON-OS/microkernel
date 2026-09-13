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

//! The SCL divider, which is arithmetic and needs no window at all.
//!
//! src/init/scl.rs:1-8 records what this cost once: "The earlier tHIGH/tLOW
//! form ran the Fast-mode bus near 500kHz, over the 400kHz budget, which a
//! real ELAN NAKs; this keeps it in spec on hardware."
//!
//! An over-clocked bus does not fail cleanly. Some transfers NAK and others
//! complete, so the touchpad reads as intermittently broken hardware and the
//! divider is the last place anyone looks.

use crate::init::scl::{fast, fs_spklen, sda_hold, standard};

const GEMINI_LAKE: u32 = 133_000_000;
const TIGER_LAKE: u32 = 100_000_000;
const SUNRISE_POINT: u32 = 120_000_000;
/// High and low count offsets the DesignWare core adds to every bit period.
const CORE_OVERHEAD: u32 = 8;

#[test]
fn the_fast_mode_bus_never_runs_over_the_four_hundred_kilohertz_budget() {
    for clk in [GEMINI_LAKE, TIGER_LAKE, SUNRISE_POINT] {
        let counts = fast(clk);
        let period = counts.hcnt + counts.lcnt + CORE_OVERHEAD;
        assert!(period >= clk / 400_000, "{clk} Hz clocks the fast-mode bus over spec");
    }
}

#[test]
fn each_input_clock_gives_the_counts_the_proven_divider_produces() {
    /*
     * Deriving these from the wrong input clock is the documented Gemini Lake
     * trap: 133 MHz counts fed a 100 MHz assumption clock the bus a third
     * fast, which is in spec for nothing.
     */
    assert_eq!((standard(GEMINI_LAKE).hcnt, standard(GEMINI_LAKE).lcnt), (658, 664));
    assert_eq!((fast(GEMINI_LAKE).hcnt, fast(GEMINI_LAKE).lcnt), (159, 165));
    assert_eq!((standard(TIGER_LAKE).hcnt, standard(TIGER_LAKE).lcnt), (493, 499));
    assert_eq!((fast(TIGER_LAKE).hcnt, fast(TIGER_LAKE).lcnt), (118, 124));
    assert_eq!((standard(SUNRISE_POINT).hcnt, standard(SUNRISE_POINT).lcnt), (593, 599));
    assert_eq!((fast(SUNRISE_POINT).hcnt, fast(SUNRISE_POINT).lcnt), (143, 149));
}

#[test]
fn a_clock_too_slow_to_divide_still_yields_counts_the_core_will_accept() {
    /*
     * The subtraction underflows below roughly 2 MHz. Saturating to zero and
     * programming a zero count stops the clock entirely, so the floors are
     * what keep a misreported input clock from bricking the bus.
     */
    let counts = standard(1_000_000);
    assert_eq!((counts.hcnt, counts.lcnt), (6, 8));
}

#[test]
fn the_spike_filter_and_sda_hold_are_never_programmed_as_zero() {
    /*
     * Zero spike length disables glitch suppression, and zero SDA hold puts
     * the data edge on the clock edge, which reads back as random NAKs.
     */
    for clk in [GEMINI_LAKE, TIGER_LAKE, SUNRISE_POINT, 1_000_000] {
        assert!(fs_spklen(clk) >= 1, "{clk} Hz gave a zero spike length");
        assert!(sda_hold(clk) >= 1, "{clk} Hz gave a zero SDA hold");
    }
    assert_eq!((fs_spklen(GEMINI_LAKE), sda_hold(GEMINI_LAKE)), (13, 39));
}
