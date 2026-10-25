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
//! IO power (PWR_WELL_CTL_DDI2, Vol 2c pp. 1072 to 1075), the lanes a link
//! powers (PORT_CL_DW10, p. 888) and the DDI buffer (DDI_BUF_CTL, p. 353).

use crate::model::clock::Steps;
use crate::model::regs::Regs;
use nonos_intel_xelp_display::phy::lanes::power_lanes;
use nonos_intel_xelp_display::phy::power::io_power_set;
use nonos_intel_xelp_display::regs::clka::Combo;

const PWR: u32 = 0x45454;

#[test]
fn io_power_on_and_off() {
    let mut m = Regs::new(Some(1));
    io_power_set(&mut m, &Steps::new(1), Combo::C, true).unwrap();
    assert_eq!(m.get(PWR), 0b11_0000);
    io_power_set(&mut m, &Steps::new(1), Combo::C, false).unwrap();
    assert_eq!(m.get(PWR), 0);
}

#[test]
fn io_power_in_flight_or_stuck() {
    let mut m = Regs::new(None);
    m.set(PWR, 1 << 3); // B asked on, not yet on
    assert!(io_power_set(&mut m, &Steps::new(1), Combo::B, false).is_err());
    assert!(m.writes.is_empty());
    let mut m = Regs::new(None);
    assert!(io_power_set(&mut m, &Steps::new(5), Combo::A, true).is_err());
}

#[test]
fn lanes_powered() {
    let mut m = Regs::new(Some(0));
    for (lanes, code) in [(4, 0b0000), (2, 0b1100), (1, 0b1110)] {
        power_lanes(&mut m, Combo::A, lanes).unwrap();
        assert_eq!((m.get(0x162028) >> 4) & 0xF, code);
    }
    assert!(power_lanes(&mut m, Combo::A, 3).is_err());
}
