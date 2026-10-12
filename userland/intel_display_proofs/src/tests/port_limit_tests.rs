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

//! Ports the driver will not train, or trains only at firmware's rate.

use super::port_tests::board;
use crate::dp::caps::Caps;
use crate::dp::train::{train, TrainError};
use crate::engine::aux_regs::Ddi;
use crate::engine::ddi::buf_trans;
use crate::engine::dpll::DPLL_CTRL1;
use crate::engine::port::Port;
use crate::model::Ticks;

#[test]
fn a_port_firmware_gave_no_translations_is_refused() {
    let (mut m, _) = board(Ddi::B);
    m.set(buf_trans(1, 0), 0);
    let clock = Ticks::new();
    let mut p = Port { mmio: &mut m, ddi: Ddi::B, dpll: 1, enhanced: true, clock: &clock };
    let caps = Caps::read(&mut p).unwrap().unwrap();
    assert_eq!(train(&mut p, &caps, (5400, 4)), Err(TrainError::Transmitter));
}

#[test]
fn edp_on_the_boot_dpll_trains_only_at_firmwares_rate() {
    let (mut m, _) = board(Ddi::A);
    m.set(DPLL_CTRL1, 0b000_011);
    let clock = Ticks::new();
    let mut p = Port { mmio: &mut m, ddi: Ddi::A, dpll: 0, enhanced: true, clock: &clock };
    let caps = Caps::read(&mut p).unwrap().unwrap();
    assert_eq!(train(&mut p, &caps, (2700, 4)).map(|t| t.mbps), Ok(2700));
    assert_eq!(train(&mut p, &caps, (5400, 4)), Err(TrainError::Transmitter));
    assert_eq!(m.get(DPLL_CTRL1) & 0x3F, 0b000_011, "the boot DPLL untouched");
}
