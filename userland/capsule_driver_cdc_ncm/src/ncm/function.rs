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

//! Finding an NCM function in a configuration (NCM 1.0, section 5): a
//! communications interface of subclass 0x0D, protocol 0, whose Union names
//! a data interface, with the Ethernet Networking and NCM functional
//! descriptors that cdc_ncm_bind_common requires, and the alternate setting
//! of the data interface that carries the bulk pipes.

use nonos_usbnet::desc::{find_functional, interfaces, union_data, CLASS_CDC_DATA, CLASS_COMM};
use nonos_usbnet::Pipes;

const SUBCLASS_NCM: u8 = 0x0D;
/// USB_CDC_PROTO_NONE: the only protocol Linux cdc_devs[] matches for NCM.
const PROTOCOL_NONE: u8 = 0x00;
/// The Ethernet Networking functional descriptor (CDC ECM 1.2, 5.4).
const ETHERNET_FUNCTIONAL: u8 = 0x0F;
/// The NCM functional descriptor (NCM 1.0, 5.2.1; USB_CDC_NCM_TYPE).
const NCM_FUNCTIONAL: u8 = 0x1A;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NcmFunction {
    pub comm: u8,
    pub data: u8,
    pub data_alt: u8,
    pub pipes: Pipes,
    /// The string descriptor index of the MAC address.
    pub mac_index: u8,
    /// wMaxSegmentSize of the Ethernet Networking descriptor.
    pub max_segment: u16,
    /// bmNetworkCapabilities of the NCM functional descriptor.
    pub caps: u8,
}

pub fn find_ncm(raw: &[u8]) -> Option<NcmFunction> {
    let ifs = interfaces(raw);
    let ncm = |i: &&nonos_usbnet::desc::Interface| {
        i.class == CLASS_COMM && i.subclass == SUBCLASS_NCM && i.protocol == PROTOCOL_NONE
    };
    ifs.iter().filter(ncm).filter(|c| c.alt == 0).find_map(|c| {
        let data = union_data(raw, c.number)?;
        let eth = find_functional(raw, c.number, ETHERNET_FUNCTIONAL).filter(|d| d.len() >= 13)?;
        let func = find_functional(raw, c.number, NCM_FUNCTIONAL).filter(|d| d.len() >= 6)?;
        let on = ifs
            .iter()
            .find(|i| i.number == data && i.class == CLASS_CDC_DATA && i.has_bulk_pair())?;
        let max_segment = u16::from_le_bytes([eth[8], eth[9]]);
        let (data_alt, pipes, mac_index, caps) = (on.alt, on.pipes, eth[3], func[5]);
        Some(NcmFunction { comm: c.number, data, data_alt, pipes, mac_index, max_segment, caps })
    })
}
