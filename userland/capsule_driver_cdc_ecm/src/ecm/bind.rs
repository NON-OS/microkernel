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

//! Binding an ECM device, in the order Linux cdc_ether and usbnet take:
//! the configuration that has an ECM function, its data interface's
//! alternate setting with the bulk pipes, then the packet filter. The
//! station address is the one drawn for the run; the iMACAddress string
//! is never read.

use nonos_usbnet::desc::config_header;
use nonos_usbnet::{Bind, Bus, Found, Setup, Station, Taken};

use super::function::{find_ecm, EcmFunction};
use super::link::Ecm;
use super::vendor::left_to_vendor_driver;

/// SET_ETHERNET_PACKET_FILTER (CDC ECM 1.2, 6.2.4): directed, broadcast
/// and all multicast, as usbnet_cdc_update_filter sets with IFF_ALLMULTI,
/// and promiscuous. ECM has no request that gives the device an address,
/// so one that filters on its own iMACAddress would drop every frame sent
/// to the drawn address unless its filter is open.
const SET_ETHERNET_PACKET_FILTER: u8 = 0x43;
const FILTER: u16 = 0x0004 | 0x0008 | 0x0002 | 0x0001;

pub fn bind<B: Bus>(mut bus: B, found: &Found, drawn: Station) -> Bind<Ecm<B>> {
    if left_to_vendor_driver(found.info.vendor, found.info.product) {
        return Bind::NotOurs;
    }
    let pick = |raw: &alloc::vec::Vec<u8>| Some((config_header(raw)?.1, find_ecm(raw)?));
    let Some((value, f)) = found.configs.iter().find_map(pick) else { return Bind::NotOurs };
    match start(&mut bus, value, &f) {
        Ok(taken) => Bind::Ours(Ecm::new(bus, drawn.bytes(), taken, f.pipes)),
        Err((what, e)) => Bind::Failed(what, e),
    }
}

/// The controller is given the pipes before the device is told to use
/// them (xHCI 1.2, 4.3.5), as for the mass-storage driver.
fn start<B: Bus>(bus: &mut B, value: u8, f: &EcmFunction) -> Result<Taken, (&'static str, i32)> {
    bus.configure_bulk(&f.pipes).map_err(|e| ("bulk pipes refused", e))?;
    let config = Setup::set_configuration(value);
    bus.control_out(config, &[]).map_err(|e| ("SET_CONFIGURATION refused", e))?;
    let data_on = Setup::set_interface(f.data, f.data_alt);
    bus.control_out(data_on, &[]).map_err(|e| ("SET_INTERFACE refused", e))?;
    // A device that does not filter answers with a STALL, as Linux expects
    // and does not look at; the log then says the address is unconfirmed.
    let filter = Setup::class(false, SET_ETHERNET_PACKET_FILTER, FILTER, f.comm);
    match bus.control_out(filter, &[]) {
        Ok(()) => Ok(Taken::Promiscuous),
        Err(_) => Ok(Taken::Unconfirmed),
    }
}
