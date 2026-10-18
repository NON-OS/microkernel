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

//! What follows INITIALIZE (Remote NDIS 1.0, 3.1): the run's station
//! address, offered as OID_802_3_CURRENT_ADDRESS, then the packet filter,
//! without which an RNDIS device passes no data, opened to every frame.
//! The device's permanent address is never asked for.

use nonos_usbnet::{Bus, Station, Taken};

use super::control::Control;
use super::message::{FILTER, OID_802_3_CURRENT_ADDRESS, OID_GEN_CURRENT_PACKET_FILTER};
use super::set::set_msg;

pub(super) fn configure<B: Bus>(
    bus: &mut B,
    ctl: &mut Control,
    reply: &mut [u8],
    drawn: Station,
) -> Result<Taken, (&'static str, i32)> {
    let mut address = set_msg(OID_802_3_CURRENT_ADDRESS, &drawn.bytes());
    let given = ctl.command(bus, &mut address, reply).is_ok();
    let mut filter = set_msg(OID_GEN_CURRENT_PACKET_FILTER, &FILTER.to_le_bytes());
    ctl.command(bus, &mut filter, reply).map_err(|e| ("RNDIS packet filter refused", e))?;
    Ok(if given { Taken::Programmed } else { Taken::Promiscuous })
}
