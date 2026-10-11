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

//! What a bring-up holds of the broker once the claim is through: the claim
//! epoch, the bus master window, and each listed channel's two windows.

use nonos_libc::{mk_device_release, mk_pio_release};

use crate::constants::pci::CHANNELS;

#[derive(Clone, Copy, Debug)]
pub struct ChannelGrants {
    pub cmd: u64,
    pub ctl: u64,
    /// The control window's size, which places the register in it.
    pub ctl_ports: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct Grants {
    pub device_id: u64,
    pub epoch: u64,
    pub bm: u64,
    pub channels: [Option<ChannelGrants>; CHANNELS],
}

impl Grants {
    /// Give back every port grant and the claim.
    pub fn release(&self) {
        for c in self.channels.iter().flatten() {
            let _ = mk_pio_release(c.ctl);
            let _ = mk_pio_release(c.cmd);
        }
        let _ = mk_pio_release(self.bm);
        let _ = mk_device_release(self.device_id);
    }
}
