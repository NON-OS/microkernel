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

//! The station address from AX_NODE_ID, as ax88179_get_mac_addr reads it
//! and writes it back.

use nonos_usbnet::desc::usable;
use nonos_usbnet::xhci::E_INVAL;
use nonos_usbnet::Bus;

use super::access::{read_mac, write_mac, Step};
use super::regs::NODE_ID;

/// Linux takes a random address when AX_NODE_ID holds none; drawing one
/// needs the Crypto capability this capsule does not hold, so the bind
/// stops here and says why.
pub const NO_STATION: &str = "AX_NODE_ID holds no station address; no random one without Crypto";

pub(super) fn station<B: Bus>(bus: &mut B) -> Result<[u8; 6], Step> {
    let mut mac = [0u8; 6];
    read_mac(bus, NODE_ID, &mut mac, "AX_NODE_ID unread")?;
    if !usable(mac) {
        return Err((NO_STATION, E_INVAL));
    }
    write_mac(bus, NODE_ID, &mac, "AX_NODE_ID write refused")?;
    Ok(mac)
}
