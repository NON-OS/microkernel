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

//! Finding an ECM function in a configuration (CDC ECM 1.2, section 3): a
//! communications interface of subclass 0x06 whose Union names a data
//! interface, and the alternate setting of that one that carries the bulk
//! pipes, as Linux usbnet_generic_cdc_bind and usbnet_get_endpoints read
//! them.

use nonos_usbnet::desc::{find_functional, interfaces, union_data, CLASS_CDC_DATA, CLASS_COMM};
use nonos_usbnet::Pipes;

const SUBCLASS_ECM: u8 = 0x06;
/// The Ethernet Networking functional descriptor (CDC ECM 1.2, 5.4).
const ETHERNET_FUNCTIONAL: u8 = 0x0F;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EcmFunction {
    pub comm: u8,
    pub data: u8,
    pub data_alt: u8,
    pub pipes: Pipes,
    /// The string descriptor index of the MAC address.
    pub mac_index: u8,
}

pub fn find_ecm(raw: &[u8]) -> Option<EcmFunction> {
    let ifs = interfaces(raw);
    let comms = ifs.iter().filter(|i| i.class == CLASS_COMM && i.subclass == SUBCLASS_ECM);
    comms.filter(|c| c.alt == 0).find_map(|c| {
        let data = union_data(raw, c.number)?;
        let eth = find_functional(raw, c.number, ETHERNET_FUNCTIONAL).filter(|d| d.len() >= 13)?;
        let on = ifs
            .iter()
            .find(|i| i.number == data && i.class == CLASS_CDC_DATA && i.has_bulk_pair())?;
        Some(EcmFunction {
            comm: c.number,
            data,
            data_alt: on.alt,
            pipes: on.pipes,
            mac_index: eth[3],
        })
    })
}
