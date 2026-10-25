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

//! The VMBus root, a host with channels, and the core up on it.

use nonos_libc::{set_devices, DeviceRecord, BUS_KIND_VIRT};

use crate::guid::Guid;
use crate::model::{Channel, Model};
use crate::setup::{self, Driver};

pub const ROOT: u64 = 0x1_0000_0003;
pub const GRANT: u64 = 0x0010_0000;

pub const NET: Guid =
    Guid::new(0xf861_5163, 0xdf3e, 0x46c5, [0x91, 0x3f, 0xf2, 0xd2, 0xf9, 0x65, 0xed, 0x0e]);
pub const SCSI: Guid =
    Guid::new(0xba61_63d9, 0x04a1, 0x4d29, [0xb6, 0x05, 0x72, 0xe2, 0xff, 0xb1, 0xdc, 0x7f]);
pub const IDE: Guid =
    Guid::new(0x3241_2632, 0x86cb, 0x44a2, [0x9b, 0x5c, 0x50, 0xd1, 0x41, 0x73, 0x54, 0xf5]);
pub const KBD: Guid =
    Guid::new(0xf912_ad6d, 0x2b17, 0x48ea, [0xbd, 0x65, 0xf9, 0x27, 0xa6, 0x1c, 0x76, 0x84]);
pub const HEARTBEAT: Guid =
    Guid::new(0x5716_4f39, 0x9115, 0x4e78, [0xab, 0x55, 0x38, 0x2f, 0x3b, 0xd5, 0x42, 0x2d]);

pub fn root() {
    set_devices(vec![DeviceRecord {
        device_id: ROOT,
        bus_kind: BUS_KIND_VIRT,
        vendor: 0x1414,
        device: 0x5642,
    }]);
}

pub fn chan(relid: u32, interface: Guid, sub: u16) -> Channel {
    let mut instance = [0u8; 16];
    instance[0] = relid as u8;
    instance[15] = 0xA5;
    Channel { relid, interface: interface.0, instance, sub, connection: 0x1000 + relid }
}

/// A Hyper-V guest's usual channels.
pub fn usual() -> Vec<Channel> {
    vec![
        chan(1, NET, 0),
        chan(2, SCSI, 0),
        chan(3, KBD, 0),
        chan(4, HEARTBEAT, 0),
        chan(5, SCSI, 1),
    ]
}

pub fn host(channels: Vec<Channel>) -> Model {
    nonos_libc::reset_channels();
    let m = Model::new(channels);
    m.install();
    m
}

pub fn up(channels: Vec<Channel>) -> (Driver, Model) {
    let m = host(channels);
    (setup::run(ROOT).expect("core up"), m)
}
