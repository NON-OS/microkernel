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

//! Which interface carries an RNDIS function's data. The Union descriptor
//! says, as usbnet_generic_cdc_bind reads it. Some Android phones have no
//! Union or one naming interfaces that do not exist; Linux then takes
//! interface 1 for a control interface 0. Before that last guess, the call
//! management descriptor's bDataInterface is taken when it names an
//! interface with bulk pipes, since it is the device's own word.

use nonos_usbnet::desc::{find_functional, union_data, Interface, CLASS_CDC_DATA};

/// The CDC Call Management functional descriptor (CDC PSTN 1.2, 5.3.1).
const CALL_MANAGEMENT: u8 = 0x01;

pub(super) fn data_interface(raw: &[u8], ifs: &[Interface], comm: u8) -> Option<u8> {
    let exists = |n: u8| ifs.iter().any(|i| i.number == n);
    let carries = |n: u8| ifs.iter().any(|i| i.number == n && i.has_bulk_pair());
    if let Some(data) = union_data(raw, comm).filter(|&d| exists(d) && d != comm) {
        // A Union that names a real interface is trusted, and that one
        // must be CDC data (usbnet_generic_cdc_bind, "slave class").
        let cdc = ifs.iter().any(|i| i.number == data && i.class == CLASS_CDC_DATA);
        return cdc.then_some(data);
    }
    let call = find_functional(raw, comm, CALL_MANAGEMENT).filter(|d| d.len() >= 5);
    if let Some(data) = call.map(|d| d[4]).filter(|&d| carries(d) && d != comm) {
        return Some(data);
    }
    (comm == 0 && carries(1)).then_some(1)
}
