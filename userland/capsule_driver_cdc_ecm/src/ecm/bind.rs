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

//! Binding an ECM device (CDC ECM 1.2, as recalled, unverified): the first
//! configuration whose functional descriptors make an ECM function, its
//! descriptors judged, every interface of it held with the host, then the
//! device configured, the data alternate turned on, the station address
//! read and the packet filter set. Each step that stops the bind is named.

use alloc::vec::Vec;

use nonos_usbnet::desc::config_header;
use nonos_usbnet::xhci::E_IO;
use nonos_usbnet::{Bind, Bus, Found, Setup};

use super::claim::{hold, Claim};
use super::function::{find_ecm, EcmFunction};
use super::link::{Ecm, RX_LEN};
use super::mac::station;
use super::segment::segment;

/// SetEthernetPacketFilter (CDC ECM 1.2, 6.2.4, as recalled, unverified):
/// directed (04h), broadcast (08h) and all multicast (02h). The stack keeps
/// no multicast list to hand the device, so every group frame comes up.
const SET_ETHERNET_PACKET_FILTER: u8 = 0x43;
const FILTER: u16 = 0x0004 | 0x0008 | 0x0002;

pub fn bind<B: Bus + Claim>(mut bus: B, found: &Found) -> Bind<Ecm<B>> {
    let pick = |raw: &Vec<u8>| Some((config_header(raw)?.1, find_ecm(raw)?));
    let mut ecm = found.configs.iter().filter_map(|raw| Some((pick(raw)?, raw)));
    let Some(((value, f), raw)) = ecm.next() else { return Bind::NotOurs };
    let judged = segment(f.max_segment, RX_LEN).map_err(|w| (w, E_IO));
    let held = judged.and_then(|_| hold(&mut bus, found.port, raw));
    match held.and_then(|_| start(&mut bus, value, &f)) {
        Ok(mac) => {
            // Without its notifications the link reads up, as from bind.
            let notify = f.notify.and_then(|ep| bus.configure_interrupt(ep).ok());
            Bind::Ours(Ecm::new(bus, mac, &f, notify))
        }
        Err((what, e)) => Bind::Failed(what, e),
    }
}

type Step<T> = Result<T, (&'static str, i32)>;

fn start<B: Bus>(bus: &mut B, value: u8, f: &EcmFunction) -> Step<[u8; 6]> {
    // The controller is given the pipes before the device is told to use
    // them (xHCI 1.2, 4.3.5, as recalled, unverified).
    bus.configure_bulk(&f.pipes).map_err(|e| ("bulk pipes refused", e))?;
    let config = Setup::set_configuration(value);
    bus.control_out(config, &[]).map_err(|e| ("SET_CONFIGURATION refused", e))?;
    let data_on = Setup::set_interface(f.data, f.data_alt);
    bus.control_out(data_on, &[]).map_err(|e| ("SET_INTERFACE refused", e))?;
    let mac = station(bus, f.mac_index)?;
    // A device that refuses the filter still carries frames, unfiltered:
    // the stack drops what is not its own, so the bind goes on.
    let filter = Setup::class(false, SET_ETHERNET_PACKET_FILTER, FILTER, f.comm);
    let _ = bus.control_out(filter, &[]);
    Ok(mac)
}
