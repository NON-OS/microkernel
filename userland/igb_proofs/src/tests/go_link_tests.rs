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

//! Go Link Disconnect, cleared on the 82580 and later: while it is set the
//! PHY keeps the link down.

use crate::bring::link;
use crate::facts::mac::PHPM_GO_LINKD;
use crate::facts::parts::Part;
use crate::facts::regs::REG_PHPM;

use super::model::{regs, window};

#[test]
fn go_link_disconnect_is_cleared_on_the_macs_that_have_it() {
    for part in [Part::I82580, Part::I350, Part::I210, Part::I211] {
        let bar = window();
        bar.present32(REG_PHPM, PHPM_GO_LINKD | 0x7);
        link::run(&regs(&bar), part);
        assert_eq!(bar.wrote32(REG_PHPM), 0x7, "{part:?}");
    }
    assert_eq!((REG_PHPM, PHPM_GO_LINKD), (0x0E14, 0x20), "E1000_82580_PHY_POWER_MGMT");
}
