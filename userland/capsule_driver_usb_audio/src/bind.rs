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

//! Whether a device is a speaker this driver plays, and setting it up:
//! its configuration, its playback alternate setting (USB Audio 1.0 or
//! 2.0), its rate, and the isoch endpoint on driver.xhci0.

use nonos_usbnet::bus::Bus;
use nonos_usbnet::setup::Setup;
use nonos_usbnet::xhci::{XhciBus, E_INVAL};
use nonos_usbnet::{Bind, Found};

use crate::rate::rate;
use crate::uac::{outputs, playback, timing, Outputs, Timing};
use crate::xhci::{configure_isoch, port_speed};

pub struct Speaker {
    pub timing: Timing,
    pub outputs: Outputs,
    pub ids: (u16, u16),
}

pub fn bind(mut bus: XhciBus, found: &Found) -> Bind<Speaker> {
    let chosen = found.configs.iter().find_map(|c| Some((c, c.get(5).copied()?, playback(c)?)));
    let Some((raw, config, p)) = chosen else { return Bind::NotOurs };
    let speed = match port_speed(bus.xhci, found.port) {
        Ok(s) => s,
        Err(e) => return Bind::Failed("port speed unread", e),
    };
    let Some(timing) = timing(speed, p.b_interval, p.max_packet, p.channels) else {
        return Bind::Failed("no 48 kHz packet fits its endpoint", E_INVAL);
    };
    if let Err(e) = bus.control_out(Setup::set_configuration(config), &[]) {
        return Bind::Failed("set configuration", e);
    }
    if let Err(e) = bus.control_out(Setup::set_interface(p.interface, p.alt), &[]) {
        return Bind::Failed("set interface", e);
    }
    if let Err((why, e)) = rate(&mut bus, raw, &p) {
        return Bind::Failed(why, e);
    }
    if let Err(e) = configure_isoch(bus.xhci, bus.slot, &p) {
        return Bind::Failed("configure isoch endpoint", e);
    }
    let ids = (found.info.vendor, found.info.product);
    Bind::Ours(Speaker { timing, outputs: outputs(raw), ids })
}
