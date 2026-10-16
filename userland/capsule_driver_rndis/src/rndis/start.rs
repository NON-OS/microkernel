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

//! Bringing an RNDIS function up (Remote NDIS 1.0, 3.1): its pipes given
//! to the controller, the configuration and alternate setting selected,
//! INITIALIZE, then the station address and packet filter (configure.rs),
//! and a HALT for a failure after INITIALIZE.

use nonos_usbnet::xhci::{CONTROL_MAX, E_IO};
use nonos_usbnet::{Bus, Setup};

use super::configure::configure;
use super::control::Control;
use super::function::RndisFunction;
use super::halt::halt;
use super::init::{init_done, init_msg, Limits};
use super::link::RX_LEN;

/// The station address, the device's limits and the media state it
/// indicated while the bind waited, if any.
pub(super) type Up = ([u8; 6], Limits, Option<bool>);

/// The controller is given the pipes before the device is told to use
/// them (xHCI 1.2, 4.3.5). SET_CONFIGURATION selects alternate setting 0
/// of every interface (USB 2.0, 9.1.1.5), so SET_INTERFACE is sent only
/// when the data pipes sit on another one.
pub(super) fn start<B: Bus>(
    bus: &mut B,
    value: u8,
    f: &RndisFunction,
) -> Result<Up, (&'static str, i32)> {
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
        Ok((mac, limits, ctl.media))
    });
    if up.is_err() {
        halt(bus, f.comm);
    }
    up
}
