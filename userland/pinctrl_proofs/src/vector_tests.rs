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

use nonos_pinctrl::{resolve, Layout, LAYOUTS};

/// The layout Linux calls `name`, for the proofs that pick one by platform.
pub(crate) fn layout(name: &str) -> &'static Layout {
    LAYOUTS.iter().map(|(_, l)| *l).find(|l| l.name == name).expect("layout")
}

/// Firmware pin -> Linux pin number and its community window. The firmware
/// number is the group's gpio_base plus the pad's place in the group; the
/// Linux pin is the one its pins[] array lists at that place (named in the
/// comment), so these pin the table to Linux, not to itself.
const VECTORS: &[(&str, u32, u32, u8)] = &[
    ("Sunrise Point-LP", 103, 103, 1), // GPP_E7 CPU_GP_1
    ("Sunrise Point-LP", 31, 31, 0),   // GPP_B7 SRCCLKREQB_2
    ("Cannon Point-LP", 263, 188, 2),  // GPP_C7 SML1DATA
    ("Cannon Point-LP", 295, 212, 2),  // GPP_E7 CPU_GP_1
    ("Cannon Point-LP", 103, 75, 1),   // GPP_D7 ISH_I2C1_SDA
    ("Ice Lake-LP", 231, 160, 2),      // GPP_C7 SML1DATA
    ("Ice Lake-LP", 39, 15, 0),        // GPP_B7 ISH_I2C1_SDA
    ("Tiger Lake-LP", 327, 233, 2),    // GPP_E7 CPU_GP_1
    ("Tiger Lake-LP", 263, 178, 2),    // GPP_C7 SML1DATA
    ("Tiger Lake-LP", 135, 82, 1),     // GPP_H7 I2C3_SCL
    ("Alder Lake-N", 327, 231, 2),     // GPP_E7 GPPC_E_7
    ("Alder Lake-N", 295, 200, 2),     // GPP_F7 GPPC_F_7
    ("Jasper Lake", 327, 7, 0),        // GPP_F7 EMMC_CMD
    ("Jasper Lake", 295, 208, 2),      // GPP_E7 ISH_GP_3
    ("Meteor Lake-P", 359, 211, 4),    // GPP_B7 GPP_B_7
    ("Meteor Lake-P", 295, 191, 3),    // GPP_S7 GPP_S_7
];

#[test]
fn firmware_pins_land_on_the_pads_linux_names() {
    for &(name, gpio, pin, bar) in VECTORS {
        let l = layout(name);
        let p = resolve(l, gpio).expect(name);
        let first = u32::from(l.communities[usize::from(p.bar)].first);
        assert_eq!((p.bar, first + u32::from(p.pad)), (bar, pin), "{} pin {}", name, gpio);
    }
}
