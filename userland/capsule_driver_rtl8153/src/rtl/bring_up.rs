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

//! The bring-up of a listed adapter's chip, in order (bind.rs names it).

use nonos_usbnet::{Bus, Setup};

use super::access::{Stop, E_TIMEDOUT};
use super::autoneg::AUTONEG;
use super::claim::{hold, Claim};
use super::config::Vendor;
use super::model::Model;
use super::op::Op;
use super::pause::Pause;
use super::run::run;
use super::start::start;
use super::station::station;
use super::usb_speed::usb_speed;
use super::version::read_version;
use super::wake::wake;

type Step<T> = Result<T, (&'static str, i32)>;

fn phase<B: Bus + Pause>(
    bus: &mut B,
    page: &mut Option<u16>,
    ops: &[Op],
    what: &'static str,
) -> Step<()> {
    run(bus, page, ops).map_err(|s| match s {
        Stop::Transfer(e) => (what, e),
        Stop::TimedOut(why) => (why, E_TIMEDOUT),
    })
}

pub fn bring_up<B: Bus + Claim + Pause>(
    bus: &mut B,
    port: u8,
    v: &Vendor,
    mac: [u8; 6],
) -> Step<(Model, Option<u16>)> {
    let model = read_version(bus)?;
    hold(bus, port, v.raw)?;
    // The controller is given the pipes before the device is told to use
    // them, as cdc_ecm does.
    bus.configure_bulk(&v.pipes).map_err(|e| ("bulk pipes refused", e))?;
    let config = Setup::set_configuration(v.value);
    bus.control_out(config, &[]).map_err(|e| ("SET_CONFIGURATION refused", e))?;
    let mut page = None;
    phase(bus, &mut page, &wake(model, usb_speed(v.pipes.max_packet_in)), "chip wake failed")?;
    phase(bus, &mut page, &station(mac), "station address refused")?;
    phase(bus, &mut page, &AUTONEG, "autonegotiation refused")?;
    phase(bus, &mut page, &start(model), "chip start failed")?;
    Ok((model, page))
}
