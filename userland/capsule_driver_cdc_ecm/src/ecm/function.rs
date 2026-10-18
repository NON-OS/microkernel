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

//! Finding an ECM function in one configuration (CDC ECM 1.2, 3 and 5, as
//! recalled, unverified): a communications interface of subclass 06h at
//! alternate 0, followed by a Union it masters (union.rs) and an Ethernet
//! Networking descriptor long enough for every field read, whose data
//! interface has an alternate 1 with one bulk pair (data.rs).

use nonos_usbnet::desc::{find_functional, interfaces, notify_ep, CLASS_COMM};
use nonos_usbnet::{Notify, Pipes};

use super::data::{bulk_alt, DATA_ON};
use super::union::union_slave;

const SUBCLASS_ECM: u8 = 0x06;
/// The Ethernet Networking descriptor: subtype 0Fh, 13 bytes through
/// bNumberPowerFilters; wMaxSegmentSize at 8. Its iMACAddress (at 3) is
/// never read, but a function without the whole descriptor is not ECM.
const ETHERNET: u8 = 0x0F;
const ETHERNET_LEN: usize = 13;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EcmFunction {
    pub comm: u8,
    pub data: u8,
    pub data_alt: u8,
    pub pipes: Pipes,
    /// wMaxSegmentSize as the device gives it; segment.rs judges it.
    pub max_segment: u16,
    /// The notification endpoint, when the interface has one.
    pub notify: Option<Notify>,
}

pub fn find_ecm(raw: &[u8]) -> Option<EcmFunction> {
    let ifs = interfaces(raw);
    let comms = ifs.iter().filter(|i| i.class == CLASS_COMM && i.subclass == SUBCLASS_ECM);
    comms.filter(|c| c.alt == 0).find_map(|c| {
        let data = union_slave(raw, c.number, &ifs)?;
        let eth = find_functional(raw, c.number, ETHERNET, ETHERNET_LEN)?;
        Some(EcmFunction {
            comm: c.number,
            data,
            data_alt: DATA_ON,
            pipes: bulk_alt(raw, data, &ifs)?,
            max_segment: u16::from_le_bytes([eth[8], eth[9]]),
            notify: notify_ep(raw, c.number),
        })
    })
}
