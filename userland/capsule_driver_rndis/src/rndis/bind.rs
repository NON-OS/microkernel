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

//! Binding an RNDIS device, in the order Linux generic_rndis_bind takes:
//! the configuration with an RNDIS function, INITIALIZE, the permanent
//! station address, then the packet filter. A failure after INITIALIZE
//! halts the device again, as Linux does at halt_fail_and_release.

use nonos_usbnet::desc::config_header;
use nonos_usbnet::xhci::{CONTROL_MAX, E_IO};
use nonos_usbnet::{Bind, Bus, Found, Setup};

use super::configure::configure;
use super::control::Control;
use super::function::{find_rndis, RndisFunction};
use super::halt::halt;
use super::init::{init_done, init_msg, Limits};
use super::link::{Rndis, RX_LEN};

pub fn bind<B: Bus>(mut bus: B, found: &Found) -> Bind<Rndis<B>> {
    let pick = |raw: &alloc::vec::Vec<u8>| Some((config_header(raw)?.1, find_rndis(raw)?));
    let Some((value, f)) = found.configs.iter().find_map(pick) else { return Bind::NotOurs };
    match start(&mut bus, value, &f) {
        Ok((mac, limits)) => Bind::Ours(Rndis::new(bus, mac, f.pipes, limits)),
        Err((what, e)) => Bind::Failed(what, e),
    }
}

/// The controller is given the pipes before the device is told to use
/// them (xHCI 1.2, 4.3.5). RNDIS data interfaces are not switched with
/// SET_INTERFACE unless the pipes sit on another alternate setting
/// (usbnet_get_endpoints with FLAG_NO_SETINT).
fn start<B: Bus>(
    bus: &mut B,
    value: u8,
    f: &RndisFunction,
) -> Result<([u8; 6], Limits), (&'static str, i32)> {
    bus.configure_bulk(&f.pipes).map_err(|e| ("bulk pipes refused", e))?;
    bus.control_out(Setup::set_configuration(value), &[])
        .map_err(|e| ("SET_CONFIGURATION refused", e))?;
    if f.data_alt != 0 {
        let on = Setup::set_interface(f.data, f.data_alt);
        bus.control_out(on, &[]).map_err(|e| ("SET_INTERFACE refused", e))?;
    }
    let (mut ctl, mut reply) = (Control::new(f.comm), [0u8; CONTROL_MAX]);
    let n = ctl
        .command(bus, &mut init_msg(RX_LEN as u32), &mut reply)
        .map_err(|e| ("RNDIS INITIALIZE failed", e))?;
    let up = init_done(&reply[..n]).map_err(|what| (what, E_IO)).and_then(|limits| {
        let mac = configure(bus, &mut ctl, &mut reply)?;
        Ok((mac, limits))
    });
    if up.is_err() {
        halt(bus, f.comm);
    }
    up
}
