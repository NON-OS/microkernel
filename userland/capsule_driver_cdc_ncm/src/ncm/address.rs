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

//! The drawn station address: given to a device that takes
//! SET_NET_ADDRESS (NCM 1.0, 6.2.2, as recalled, unverified), sent while
//! the data interface is in alternate setting 0 as the spec asks. Any
//! other device has its packet filter opened to every frame instead, so
//! one that filters on its own iMACAddress still passes frames to it.

use nonos_usbnet::{Bus, Setup, Station, Taken};

use super::function::NcmFunction;
use super::requests::{CAP_NET_ADDRESS, FILTER, PROMISCUOUS};
use super::requests::{SET_ETHERNET_PACKET_FILTER, SET_NET_ADDRESS};

/// Whether the device took the drawn address.
pub(super) fn give<B: Bus>(bus: &mut B, f: &NcmFunction, drawn: Station) -> bool {
    let set = Setup::class(false, SET_NET_ADDRESS, 0, f.comm);
    f.caps & CAP_NET_ADDRESS != 0 && bus.control_out(set, &drawn.bytes()).is_ok()
}

/// The packet filter (requests.rs), opened to every frame when the device
/// did not take the address. A device that does not filter answers with a
/// STALL; the frames it passes are the same, so the bind goes on.
pub(super) fn filter<B: Bus>(bus: &mut B, f: &NcmFunction, given: bool) -> Taken {
    let value = if given { FILTER } else { FILTER | PROMISCUOUS };
    let set = Setup::class(false, SET_ETHERNET_PACKET_FILTER, value, f.comm);
    match (given, bus.control_out(set, &[]).is_ok()) {
        (true, _) => Taken::Programmed,
        (false, true) => Taken::Promiscuous,
        (false, false) => Taken::Unconfirmed,
    }
}
