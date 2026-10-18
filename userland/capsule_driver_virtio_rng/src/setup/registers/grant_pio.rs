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

use crate::discover::Found;
use crate::regs::Regs;
use nonos_libc::{mk_pio_grant, PioGrantOut};

pub(super) fn grant_pio(dev: Found, epoch: u64) -> Result<Regs, &'static str> {
    let mut out = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    if mk_pio_grant(dev.device_id, epoch, dev.register_bar, 0, &mut out) < 0 {
        return Err("pio grant failed");
    }
    Ok(Regs::pio(out.grant_id))
}
