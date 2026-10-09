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

//! Binding an AX88179 or AX88178A as Linux ax88179_bind does: the vendor
//! interface's pipes, then ax88179_reset in its order. The link is read
//! afterwards, on the clock (link_poll.rs).

use nonos_usbnet::{Bind, Bus, Found, Setup};

use super::access::Step;
use super::autoneg::restart_autoneg;
use super::eee::disable_eee;
use super::function::{find_function, AxFunction};
use super::link::Ax88179;
use super::power::{auto_detach, power_up};
use super::products::listed;
use super::receive::receive_setup;
use super::station::station;

pub fn bind<B: Bus>(mut bus: B, found: &Found) -> Bind<Ax88179<B>> {
    if !listed(found.info.vendor, found.info.product) {
        return Bind::NotOurs;
    }
    let Some(f) = find_function(found) else { return Bind::NotOurs };
    match start(&mut bus, &f) {
        Ok(mac) => Bind::Ours(Ax88179::new(bus, mac, f.pipes)),
        Err((what, e)) => Bind::Failed(what, e),
    }
}

/// The controller is given the pipes before the device is told to use
/// them (xHCI 1.2, 4.3.5), as for the other USB network drivers.
fn start<B: Bus>(bus: &mut B, f: &AxFunction) -> Result<[u8; 6], Step> {
    bus.configure_bulk(&f.pipes).map_err(|e| ("bulk pipes refused", e))?;
    let config = Setup::set_configuration(f.config);
    bus.control_out(config, &[]).map_err(|e| ("SET_CONFIGURATION refused", e))?;
    // usbnet_get_endpoints selects the setting; ax88179_bind does not look
    // at the answer, and a device with one setting may stall it.
    let _ = bus.control_out(Setup::set_interface(f.interface, f.alt), &[]);
    reset(bus)
}

/// ax88179_reset, step by step. The LED setup from the EEPROM is left
/// out: the chip's own LED defaults stay.
fn reset<B: Bus>(bus: &mut B) -> Result<[u8; 6], Step> {
    power_up(bus)?;
    auto_detach(bus)?;
    let mac = station(bus)?;
    receive_setup(bus)?;
    disable_eee(bus)?;
    restart_autoneg(bus)?;
    Ok(mac)
}
