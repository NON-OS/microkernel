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

//! Binding an AX88179 or AX88178A: a listed vendor and product (ids.rs)
//! with the chip's vendor interface in one of its configurations
//! (function.rs), every interface of that configuration held with the
//! host, the pipes given to the controller, the device configured, then
//! brought up (bring_up.rs) with the run's drawn station address written
//! into it. The factory address is never read. Each step that stops the
//! bind is named; a listed device with no vendor interface is another
//! driver's (a class configuration), never failed here.

use alloc::vec::Vec;

use nonos_usbnet::desc::config_header;
use nonos_usbnet::{Bind, Bus, Found, Setup, Station};

use super::bring_up::{bring_up, Pause, Stop};
use super::claim::{hold, Claim};
use super::function::{find_ax, AxFunction};
use super::ids::is_ax;
use super::link::Ax;

pub fn bind<B: Bus + Claim + Pause>(mut bus: B, found: &Found, drawn: Station) -> Bind<Ax<B>> {
    if !is_ax(found.info.vendor, found.info.product) {
        return Bind::NotOurs;
    }
    let pick = |raw: &Vec<u8>| Some((config_header(raw)?.1, find_ax(raw)?));
    let mut ax = found.configs.iter().filter_map(|raw| Some((pick(raw)?, raw)));
    let Some(((value, f), raw)) = ax.next() else { return Bind::NotOurs };
    let held = hold(&mut bus, found.port, raw);
    match held.and_then(|_| start(&mut bus, value, &f, drawn.bytes())) {
        Ok(()) => {
            // Without its reports the link is read from the PHY (watch.rs).
            let status = f.status.and_then(|ep| bus.configure_interrupt(ep).ok());
            Bind::Ours(Ax::new(bus, drawn.bytes(), &f, status))
        }
        Err((what, e)) => Bind::Failed(what, e),
    }
}

fn start<B: Bus + Pause>(bus: &mut B, value: u8, f: &AxFunction, mac: [u8; 6]) -> Result<(), Stop> {
    // The controller is given the pipes before the device is told to use
    // them (xHCI 1.2, 4.3.5, as recalled, unverified).
    bus.configure_bulk(&f.pipes).map_err(|e| ("bulk pipes refused", e))?;
    let config = Setup::set_configuration(value);
    bus.control_out(config, &[]).map_err(|e| ("SET_CONFIGURATION refused", e))?;
    bring_up(bus, mac)
}
