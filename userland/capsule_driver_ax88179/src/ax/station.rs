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

//! The station address: the one drawn for the run, written to
//! AX_NODE_ID as ax88179_set_mac_addr writes an address, so the chip's own
//! unicast filter (AX_RX_CTL_AP) matches it. The factory value there is
//! never read (Linux ax88179_get_mac_addr does), so a chip without one
//! binds all the same.

use nonos_usbnet::{Bus, Station};

use super::access::{write_mac, Step};
use super::regs::NODE_ID;

pub(super) fn station<B: Bus>(bus: &mut B, drawn: Station) -> Result<[u8; 6], Step> {
    let mac = drawn.bytes();
    write_mac(bus, NODE_ID, &mac, "AX_NODE_ID write refused")?;
    Ok(mac)
}
