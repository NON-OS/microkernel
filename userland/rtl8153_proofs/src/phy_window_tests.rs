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

//! The PHY behind the PLA window (Linux ocp_reg_read): the window moves
//! only when the page changes.

use crate::chip::{rtl8153, RTL8153_IDS};
use crate::ocp_tests::{rd, wr};
use crate::r8153::ocp::{phy_read, Dev};

#[test]
fn the_phy_window_moves_only_when_the_page_changes() {
    let (bus, _) = rtl8153(0x5c10, RTL8153_IDS);
    let mut dev = Dev::new(bus.clone());
    // OCP_PHY_STATUS twice, then OCP_ADC_CFG on the next page.
    assert_eq!(phy_read(&mut dev, 0xa420), Ok(3));
    phy_read(&mut dev, 0xa420).unwrap();
    phy_read(&mut dev, 0xbc06).unwrap();
    let want = [
        wr(0xe86c, 0x0133, [0x00, 0xa0, 0, 0]),
        rd(0xb420, 0x0133),
        rd(0xb420, 0x0133),
        wr(0xe86c, 0x0133, [0x00, 0xb0, 0, 0]),
        rd(0xbc04, 0x01cc),
    ];
    assert_eq!(bus.0.borrow().calls, want);
}
