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

//! The station address: drawn for the run and written to PLA_IDR, which
//! the chip's unicast filter matches; PLA_BACKUP, where the factory
//! address sits, is never read, and a chip with none there binds.

use nonos_usbnet::mock::drawn;
use nonos_usbnet::Bind;

use crate::chip::{found, reads_of, PLA, RTL8153_IDS};
use crate::r8153::bind;

/// An address from Realtek's own range, as an adapter's efuse holds one.
const FACTORY: [u8; 6] = [0x00, 0xe0, 0x4c, 0x68, 0x00, 0x01];

#[test]
fn the_address_on_the_wire_is_drawn_and_programmed_never_the_chips() {
    let (bus, chip, found) = found(0x5c10, RTL8153_IDS);
    chip.borrow_mut().regs.put(PLA, 0xd000, &FACTORY);
    let Bind::Ours(nic) = bind(bus.clone(), &found, drawn()) else { panic!("not bound") };
    let mac = nonos_usbnet::Nic::mac(&nic);
    assert_ne!(mac, FACTORY);
    assert_eq!(mac[0] & 0x03, 0x02, "locally administered unicast");
    assert_eq!(chip.borrow().regs.read(PLA, 0xc000, 6), mac, "PLA_IDR holds it");
    assert_eq!(reads_of(&bus, 0xd000), 0, "PLA_BACKUP is never read");
}

#[test]
fn a_chip_whose_backup_holds_no_address_binds_all_the_same() {
    for raw in [[0u8; 6], [0x01, 0, 0, 0, 0, 0], [0xff; 6]] {
        let (bus, chip, found) = found(0x5c10, RTL8153_IDS);
        chip.borrow_mut().regs.put(PLA, 0xd000, &raw);
        assert!(matches!(bind(bus, &found, drawn()), Bind::Ours(_)), "{raw:02x?}");
    }
}
