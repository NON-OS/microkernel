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

//! The requests that bring an NCM function up (NCM 1.0, 6.2 and 7.1, as
//! recalled, unverified), each failure named for the log.

use nonos_libc::mk_idle_ms;
use nonos_usbnet::{Bus, Setup, Station, Taken};

use super::address::{filter, give};
use super::function::NcmFunction;
use super::init::init;
use super::params::NtbParams;
use super::setup::setup;

/// A pause between the setup in alternate setting 0 and selecting the
/// data alternate: some modem firmware (Sierra Wireless MC7455) fails to
/// set up the interface without it (observed, unverified).
const SETTLE_MS: u64 = 10;

pub(super) type Up = (Taken, NtbParams, usize);

/// The controller is given the pipes before the device is told to use
/// them (xHCI 1.2, 4.3.5), as for the ECM driver.
pub(super) fn start<B: Bus>(
    bus: &mut B,
    value: u8,
    f: &NcmFunction,
    drawn: Station,
) -> Result<Up, (&'static str, i32)> {
    bus.configure_bulk(&f.pipes).map_err(|e| ("bulk pipes refused", e))?;
    let config = Setup::set_configuration(value);
    bus.control_out(config, &[]).map_err(|e| ("SET_CONFIGURATION refused", e))?;
    // The setup runs with the data interface in alternate setting 0:
    // SET_NTB_FORMAT is taken only there (NCM 1.0, 6.2.5).
    let off = Setup::set_interface(f.data, 0);
    bus.control_out(off, &[]).map_err(|e| ("SET_INTERFACE 0 refused", e))?;
    let p = init(bus, f)?;
    let given = give(bus, f, drawn);
    mk_idle_ms(SETTLE_MS);
    let on = Setup::set_interface(f.data, f.data_alt);
    bus.control_out(on, &[]).map_err(|e| ("SET_INTERFACE refused", e))?;
    let rx = setup(bus, f, &p)?;
    Ok((filter(bus, f, given), p, rx))
}
