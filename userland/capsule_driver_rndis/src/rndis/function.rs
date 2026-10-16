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
//! one of the classes RNDIS devices report (below), its data interface,
//! and the alternate setting of that one with the bulk pipes.

use nonos_usbnet::desc::{find_functional, interfaces, notify_ep, Interface, CLASS_COMM};
use nonos_usbnet::{Notify, Pipes};

use super::data_iface::data_interface;

/// (class, subclass, protocol): CDC ACM with the vendor protocol (Remote
/// NDIS 1.0, USB mapping), and the wireless controller and miscellaneous
/// class Remote NDIS codes (USB-IF defined class codes, as recalled,
/// unverified).
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
    /// Its notification endpoint, when it has one.
    pub notify: Option<Notify>,
}

pub fn find_rndis(raw: &[u8]) -> Option<RndisFunction> {
    let ifs = interfaces(raw);
    let controls = ifs.iter().filter(|c| c.alt == 0 && is_rndis_control(c));
    controls.filter(|c| !acm_capable(raw, c)).find_map(|c| {
        let data = data_interface(raw, &ifs, c.number)?;
        let on = ifs.iter().find(|i| i.number == data && i.has_bulk_pair())?;
        let notify = notify_ep(raw, c.number);
        Some(RndisFunction { comm: c.number, data, data_alt: on.alt, pipes: on.pipes, notify })
    })
}

fn is_rndis_control(i: &Interface) -> bool {
    CONTROL_CLASSES.contains(&(i.class, i.subclass, i.protocol))
}

/// A communications class function whose ACM descriptor claims modem
/// capabilities is a real modem, not RNDIS: an RNDIS function reports
/// none (as recalled, unverified). Only that class is checked: the
/// wireless class codes do not define the field this way.
fn acm_capable(raw: &[u8], c: &Interface) -> bool {
    let acm = find_functional(raw, c.number, ACM_FUNCTIONAL).filter(|d| d.len() >= 4);
    c.class == CLASS_COMM && acm.is_some_and(|d| d[3] != 0)
}
