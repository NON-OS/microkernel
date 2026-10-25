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

//! The VMBus root, whose holder may use the SynIC's calls to the Hyper-V
//! host, held for the VMBus core alone; a channel's record held for the
//! driver of its interface; any other virtual-bus record refused
//! (`hold_virt`, `vmbus.rs`).

use super::hold_platform::{hold_for_platform, OTHER, PNP, PS2_KBD, SERIAL, SERIAL_BUS};
use super::hold_rule::Hold;
use super::hold_virt::*;

#[test]
fn the_vmbus_root_is_held_for_the_vmbus_core() {
    let root = hold_for_platform(BUS_VIRT, OTHER, 0, VMBUS_VENDOR, VMBUS_DEVICE);
    assert_eq!(root, Some(Hold::ForService(VMBUS_SERVICE)));
    assert_eq!(VMBUS_SERVICE, "driver.vmbus0");
    assert_eq!((BUS_VIRT, VMBUS_VENDOR, VMBUS_DEVICE), (3, 0x1414, 0x5642));
    // The class the record carries does not open it.
    for class in [0u32, SERIAL, 0x0080] {
        let hold = hold_for_platform(BUS_VIRT, class, 0, VMBUS_VENDOR, VMBUS_DEVICE);
        assert_eq!(hold, Some(Hold::ForService(VMBUS_SERVICE)), "{class:#x}");
    }
}

#[test]
fn every_other_virtual_bus_record_is_refused_until_named() {
    // Keyboard, mouse and video channels have no driver yet; a channel
    // device under another vendor is no channel.
    let others = [
        (VMBUS_VENDOR, 0),
        (0, VMBUS_DEVICE),
        (PNP, PS2_KBD),
        (0x80EE, 0xCAFE),
        (0, 0),
        (VMBUS_VENDOR, 0x4B42),
        (VMBUS_VENDOR, 0x4D53),
        (VMBUS_VENDOR, 0x5644),
        (0, CHANNEL_SCSI),
        (0x8086, CHANNEL_NETWORK),
    ];
    for (vendor, device) in others {
        let hold = hold_for_platform(BUS_VIRT, OTHER, SERIAL_BUS, vendor, device);
        assert_eq!(hold, Some(Hold::Refused), "{vendor:#x}:{device:#x}");
        assert_eq!(hold_for_virt(vendor, device), Hold::Refused);
    }
    // The root's identity on any other bus is no VMBus root.
    for bus in [0u8, 1, 5] {
        assert_eq!(hold_for_platform(bus, OTHER, 0, VMBUS_VENDOR, VMBUS_DEVICE), None, "{bus}");
    }
}

#[test]
fn a_channel_s_record_is_held_for_the_driver_of_its_interface() {
    assert_eq!((CHANNEL_NETWORK, CHANNEL_SCSI, CHANNEL_IDE), (0x4E45, 0x5343, 0x4944));
    assert_eq!((STORVSC_SERVICE, NETVSC_SERVICE), ("driver.storvsc0", "driver.netvsc0"));
    for (device, service) in [
        (CHANNEL_SCSI, STORVSC_SERVICE),
        (CHANNEL_IDE, STORVSC_SERVICE),
        (CHANNEL_NETWORK, NETVSC_SERVICE),
    ] {
        let hold = hold_for_platform(BUS_VIRT, OTHER, 0, VMBUS_VENDOR, device);
        assert_eq!(hold, Some(Hold::ForService(service)), "{device:#x}");
        // On any other bus the same numbers are no channel.
        assert_ne!(hold_for_platform(0, OTHER, 0, VMBUS_VENDOR, device), hold);
    }
}
