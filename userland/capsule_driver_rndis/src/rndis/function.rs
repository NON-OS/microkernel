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

//! Finding an RNDIS function in a configuration: a control interface of
//! one of the classes Linux rndis_host's products[] and cdc_ether's
//! is_rndis, is_wireless_rndis and is_novatel_rndis name, its data
//! interface, and the alternate setting of that one with the bulk pipes.

use nonos_usbnet::desc::{find_functional, interfaces, Interface, CLASS_COMM};
use nonos_usbnet::Pipes;

use super::data_iface::data_interface;

/// (class, subclass, protocol): CDC ACM with the vendor protocol, the
/// wireless controller "RNDIS for tethering" of Android phones, and the
/// miscellaneous class RNDIS of the Novatel USB730L.
const CONTROL_CLASSES: [(u8, u8, u8); 3] =
    [(CLASS_COMM, 0x02, 0xFF), (0xE0, 0x01, 0x03), (0xEF, 0x04, 0x01)];
/// The CDC ACM functional descriptor (CDC PSTN 1.2, 5.3.2).
const ACM_FUNCTIONAL: u8 = 0x02;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RndisFunction {
    pub comm: u8,
    pub data: u8,
    pub data_alt: u8,
    pub pipes: Pipes,
}

pub fn find_rndis(raw: &[u8]) -> Option<RndisFunction> {
    let ifs = interfaces(raw);
    let controls = ifs.iter().filter(|c| c.alt == 0 && is_rndis_control(c));
    controls.filter(|c| !acm_capable(raw, c)).find_map(|c| {
        let data = data_interface(raw, &ifs, c.number)?;
        let on = ifs.iter().find(|i| i.number == data && i.has_bulk_pair())?;
        Some(RndisFunction { comm: c.number, data, data_alt: on.alt, pipes: on.pipes })
    })
}

fn is_rndis_control(i: &Interface) -> bool {
    CONTROL_CLASSES.contains(&(i.class, i.subclass, i.protocol))
}

/// A communications class function whose ACM descriptor claims modem
/// capabilities is a real modem, not RNDIS (usbnet_generic_cdc_bind,
/// "ACM capabilities, not really RNDIS?"). Only that class is checked:
/// wireless class RNDIS uses the field for its own purposes.
fn acm_capable(raw: &[u8], c: &Interface) -> bool {
    let acm = find_functional(raw, c.number, ACM_FUNCTIONAL).filter(|d| d.len() >= 4);
    c.class == CLASS_COMM && acm.is_some_and(|d| d[3] != 0)
}
