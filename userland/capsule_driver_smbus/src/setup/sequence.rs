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

//! One bring-up attempt: claim, host on, ports granted.

use nonos_libc::{mk_pio_grant, PioGrantOut};

use super::claim::Claim;
use super::host;
use crate::discover::{Found, SMB_BAR};
use crate::pio::Host;

/// Field order is drop order: nothing here outlives the claim.
pub struct Driver {
    pub host: Host,
    pub found: Found,
    pub io_ports: u16,
    _claim: Claim,
}

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let claim = Claim::take(dev.device_id)?;
    host::enable(&claim)?;
    let mut out = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    if mk_pio_grant(claim.device_id, claim.epoch, SMB_BAR, 0, &mut out) < 0 {
        return Err("smbus: the broker refused the port grant");
    }
    let host = Host::new(out.grant_id);
    Ok(Driver { host, found: dev, io_ports: out.port_count, _claim: claim })
}
