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

//! What follows INITIALIZE in Linux generic_rndis_bind: the permanent
//! station address, which the bind cannot go on without, then the packet
//! filter, without which an RNDIS device passes no data.

use nonos_usbnet::xhci::E_IO;
use nonos_usbnet::Bus;

use super::control::Control;
use super::message::{FILTER, OID_802_3_PERMANENT_ADDRESS, OID_GEN_CURRENT_PACKET_FILTER};
use super::query::{query_msg, station, ADDRESS_PAYLOAD};
use super::set::set_msg;

pub(super) fn configure<B: Bus>(
    bus: &mut B,
    ctl: &mut Control,
    reply: &mut [u8],
) -> Result<[u8; 6], (&'static str, i32)> {
    let mut ask = query_msg(OID_802_3_PERMANENT_ADDRESS, ADDRESS_PAYLOAD);
    let n = ctl.command(bus, &mut ask, reply).map_err(|e| ("RNDIS address query failed", e))?;
    let mac = station(&reply[..n]).ok_or(("no permanent address", E_IO))?;
    let mut filter = set_msg(OID_GEN_CURRENT_PACKET_FILTER, FILTER);
    ctl.command(bus, &mut filter, reply).map_err(|e| ("RNDIS packet filter refused", e))?;
    Ok(mac)
}
