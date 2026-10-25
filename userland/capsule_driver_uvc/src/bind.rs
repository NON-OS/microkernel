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
//! Whether a device is a camera this driver serves, and setting it up:
//! its configuration selected, the streaming interface left at alternate
//! 0, so it takes no bandwidth until a stream starts.

use nonos_usbnet::bus::Bus;
use nonos_usbnet::setup::Setup;
use nonos_usbnet::xhci::XhciBus;
use nonos_usbnet::{Bind, Found};
use nonos_uvc::{camera, Camera};

pub struct Cam {
    pub cam: Camera,
    pub ids: (u16, u16),
}

pub fn bind(mut bus: XhciBus, found: &Found) -> Bind<Cam> {
    let chosen = found.configs.iter().find_map(|c| Some((c.get(5).copied()?, camera(c)?)));
    let Some((config, cam)) = chosen else { return Bind::NotOurs };
    if cam.alts.is_empty() {
        return Bind::Failed("no isochronous streaming endpoint", nonos_usbnet::xhci::E_INVAL);
    }
    if let Err(e) = bus.control_out(Setup::set_configuration(config), &[]) {
        return Bind::Failed("set configuration", e);
    }
    if let Err(e) = bus.control_out(Setup::set_interface(cam.streaming, 0), &[]) {
        return Bind::Failed("set interface 0", e);
    }
    Bind::Ours(Cam { cam, ids: (found.info.vendor, found.info.product) })
}
