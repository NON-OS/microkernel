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

//! The CDCLK limit on a pipe's pixel rate, and the mode sets that keep it.

use super::port_tests::board;
use crate::dp::caps::Caps;
use crate::engine::aux_regs::Ddi;
use crate::engine::cdclk::{fits, khz, CDCLK_CTL};
use crate::engine::error::Error;
use crate::engine::modeset::enable_dp;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::port::Port;
use crate::model::{Gen9, Ticks};

#[test]
fn the_decimal_field_reads_as_half_megahertz_steps() {
    let mut m = Gen9::new(1);
    assert_eq!(khz(&mut m), 0);
    assert!(!fits(&mut m, 25_175), "unset refuses everything");
    m.set(CDCLK_CTL, 674);
    assert_eq!(khz(&mut m), 337_500);
    assert!(fits(&mut m, 337_500) && !fits(&mut m, 337_501));
}

#[test]
fn a_mode_faster_than_firmwares_cdclk_is_refused_before_any_write() {
    let (mut m, _) = board(Ddi::B);
    m.set(CDCLK_CTL, 674);
    let clock = Ticks::new();
    let t = nonos_modes::cvt(3840, 2160, 60_000, nonos_modes::Blanking::Reduced).unwrap();
    let mut p = Port { mmio: &mut m, ddi: Ddi::B, dpll: 1, enhanced: true, clock: &clock };
    let caps = Caps::read(&mut p).unwrap().unwrap();
    m.writes.clear();
    let mut p = Port { mmio: &mut m, ddi: Ddi::B, dpll: 1, enhanced: true, clock: &clock };
    let r = enable_dp(&mut p, Transcoder::B, Pipe::B, &t, &caps, (5400, 4));
    assert_eq!(r.err(), Some(Error::Argument));
    assert!(m.writes.is_empty());
}
