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

//! Waits that end on the clock, and a refused register naming its step.

use nonos_usbnet::Bind;

use crate::chip::{PLA, RTL8153_IDS};
use crate::r8153::bind;

#[test]
fn a_chip_that_never_gets_ready_fails_on_the_clock() {
    // LINK_LIST_READY never comes: the 2 s wait of wait_oob_link_list_ready.
    let (bus, chip, found) = crate::chip::found(0x5c10, RTL8153_IDS);
    chip.borrow_mut().regs.put(PLA, 0xe84f, &[0x80]);
    let start = std::time::Instant::now();
    assert!(matches!(bind(bus, &found), Bind::Failed("link list not ready", -110)));
    assert!(start.elapsed() >= std::time::Duration::from_millis(2_000));
    // A register the chip refuses stops the bring-up at that step.
    let (bus, chip, found) = crate::chip::found(0x5c10, RTL8153_IDS);
    chip.borrow_mut().refuse = Some((0x05, 0xd404, -71));
    assert!(matches!(bind(bus, &found), Bind::Failed("RX aggregation not disabled", -71)));
}
