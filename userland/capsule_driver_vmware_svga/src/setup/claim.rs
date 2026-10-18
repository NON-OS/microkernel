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

//! Claim phase, and the port I/O grant on BAR0 the registers are reached
//! through. A refused grant gives the claim back.

use nonos_libc::{mk_device_claim, mk_device_release, mk_pio_grant, PioGrantOut};

pub fn claim(device_id: u64, port_bar: u8) -> Result<(u64, u64), &'static str> {
    let epoch = mk_device_claim(device_id);
    if epoch < 0 {
        return Err("claim failed");
    }
    let mut out = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    if mk_pio_grant(device_id, epoch as u64, port_bar, 0, &mut out) < 0 || out.port_count < 2 {
        let _ = mk_device_release(device_id);
        return Err("port grant failed");
    }
    Ok((epoch as u64, out.grant_id))
}
