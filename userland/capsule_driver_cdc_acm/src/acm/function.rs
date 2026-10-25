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
//! Finding an Abstract Control Model function in a configuration (CDC
//! PSTN 1.2, sections 3 and 5.3, as recalled, unverified): a
//! communications interface of subclass 0x02 whose Union names a data
//! interface with a bulk pair, its ACM functional descriptor's
//! capabilities, and the interrupt IN endpoint its notifications come on.

use nonos_usbnet::desc::{find_functional, interfaces, notify_ep, union_data};
use nonos_usbnet::desc::{CLASS_CDC_DATA, CLASS_COMM};
use nonos_usbnet::{Notify, Pipes};

const SUBCLASS_ACM: u8 = 0x02;
/// The Abstract Control Management functional descriptor.
const ACM_FUNCTIONAL: u8 = 0x02;

/// bmCapabilities bit 1: line coding, control line state and serial
/// state; bit 2: send break.
pub const CAP_LINE: u8 = 1 << 1;
pub const CAP_BREAK: u8 = 1 << 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AcmFunction {
    pub comm: u8,
    /// bInterfaceProtocol: 1 is a modem's AT commands (V.250).
    pub protocol: u8,
    pub capabilities: u8,
    pub data: u8,
    pub data_alt: u8,
    pub pipes: Pipes,
    pub notify: Option<Notify>,
}

pub fn find_acm(raw: &[u8]) -> Option<AcmFunction> {
    let ifs = interfaces(raw);
    let comms = ifs.iter().filter(|i| i.class == CLASS_COMM && i.subclass == SUBCLASS_ACM);
    comms.filter(|c| c.alt == 0).find_map(|c| {
        let data = union_data(raw, c.number)?;
        let on = ifs
            .iter()
            .find(|i| i.number == data && i.class == CLASS_CDC_DATA && i.has_bulk_pair())?;
        let acm = find_functional(raw, c.number, ACM_FUNCTIONAL, 4);
        Some(AcmFunction {
            comm: c.number,
            protocol: c.protocol,
            capabilities: acm.map_or(0, |d| d[3]),
            data,
            data_alt: on.alt,
            pipes: on.pipes,
            notify: notify_ep(raw, c.number),
        })
    })
}
