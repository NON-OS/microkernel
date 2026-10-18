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

//! Telling the device where the shared area is and activating it, then the
//! receive filter. ACTIVATE_DEV answers in CMD: zero, or the reason the
//! device refused the layout. A refusal leaves the shared address cleared,
//! as Linux leaves it, so nothing points at memory about to be given back.

use crate::constants::cmd::{
    ACTIVATE_DEV, UPDATE_MAC_FILTERS, UPDATE_RX_MODE, UPDATE_VLAN_FILTERS,
};
use crate::constants::MAC_LEN;
use crate::regs::Regs;

pub fn run(regs: &Regs, shared_pa: u64, mac: &[u8; MAC_LEN]) -> Result<(), &'static str> {
    regs.set_mac(mac);
    regs.set_shared(shared_pa);
    if regs.command(ACTIVATE_DEV) != 0 {
        regs.set_shared(0);
        return Err("device refused activation");
    }
    regs.mask();
    Ok(())
}

/// The filter in the shared area takes effect only on these commands, the
/// order Linux's vmxnet3_set_mc issues them in.
pub fn apply_filter(regs: &Regs) {
    regs.command(UPDATE_RX_MODE);
    regs.command(UPDATE_VLAN_FILTERS);
    regs.command(UPDATE_MAC_FILTERS);
}
