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
//! The combo PHY and DDI registers at the addresses Vol 2c lists for them.

use nonos_intel_xelp_display::regs::clka::Combo;
use nonos_intel_xelp_display::regs::ddi::{self, buf_ctl, io_power, phy_misc};
use nonos_intel_xelp_display::regs::phy::{self, group, lane, port};

#[test]
fn listed_instances() {
    // A sample of Vol 2c's instance rows, one per register kind and port.
    let cases = [
        (port(Combo::A, phy::COMP_DW0), 0x162100), // p. 896
        (port(Combo::B, phy::COMP_DW3), 0x6C10C),  // p. 899
        (port(Combo::C, phy::COMP_DW9), 0x160124), // p. 902
        (port(Combo::A, phy::COMP_DW10), 0x162128),
        (port(Combo::B, phy::COMP_DW1), 0x6C104),
        (port(Combo::C, phy::COMP_DW8), 0x160120),
        (port(Combo::A, phy::CL_DW5), 0x162014),  // p. 885
        (port(Combo::C, phy::CL_DW10), 0x160028), // p. 887
        (group(Combo::B, phy::PCS_DW1), 0x6C604), // p. 904
        (lane(Combo::A, 0, phy::PCS_DW1), 0x162804),
        (lane(Combo::A, 1, phy::TX_DW2), 0x162988), // p. 935
        (lane(Combo::B, 2, phy::TX_DW4), 0x6CA90),  // p. 938
        (group(Combo::A, phy::TX_DW5), 0x162694),   // p. 941
        (lane(Combo::A, 0, phy::TX_DW7), 0x16289C), // p. 948
        (lane(Combo::C, 3, phy::TX_DW8), 0x160BA0), // p. 951
        (group(Combo::C, phy::TX_DW8), 0x1606A0),
    ];
    for (got, want) in cases {
        assert_eq!(got, want, "{want:#x}");
    }
}

#[test]
fn ddi_registers() {
    // DDI_BUF_CTL p. 352; PHY_MISC p. 664; PWR_WELL_CTL_DDI2 p. 1071.
    assert_eq!([Combo::A, Combo::B, Combo::C].map(buf_ctl), [0x64000, 0x64100, 0x64200]);
    assert_eq!([Combo::A, Combo::B, Combo::C].map(phy_misc), [0x64C00, 0x64C04, 0x64C08]);
    assert_eq!(ddi::PWR_WELL_CTL_DDI2, 0x45454);
    // Requests 1, 3, 5 and states 0, 2, 4 (pp. 1074, 1075).
    assert_eq!([Combo::A, Combo::B, Combo::C].map(io_power), [(2, 1), (8, 4), (32, 16)]);
    assert_eq!((ddi::BUF_ENABLE, ddi::BUF_IDLE, ddi::BUF_WIDTH), (1 << 31, 1 << 7, 0xE));
    assert_eq!(ddi::MISC_COMP_PWR_DOWN, 1 << 23);
}
