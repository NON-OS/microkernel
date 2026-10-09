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

use super::types::Transport;
use crate::constants::{LEG_MAC, LEG_QUEUE_NOTIFY};

impl Transport {
    /// Tell the device queue `queue` has new buffers.
    pub fn notify(self, queue: u16) {
        match self {
            Self::Legacy(regs) => unsafe { regs.w16(LEG_QUEUE_NOTIFY, queue) },
            Self::Modern(m) => {
                if let Some(&at) = m.doorbells.get(queue as usize) {
                    m.notify.w16(at, queue);
                }
            }
        }
    }

    /// A 16-bit field of struct virtio_net_config, by its offset there.
    pub fn config_r16(self, off: usize) -> u16 {
        match self {
            // No MSI-X vector is enabled, so the legacy device-specific
            // config starts right at the MAC.
            Self::Legacy(regs) => unsafe { regs.r16(LEG_MAC + off) },
            Self::Modern(m) => m.device.r16(off),
        }
    }
}
