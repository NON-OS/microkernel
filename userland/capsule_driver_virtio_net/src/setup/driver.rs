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

use nonos_libc::mk_device_release;

use crate::constants::MAC_LEN;
use crate::queue::{RxQueue, TxQueue};
use crate::transport::Transport;

pub struct Driver {
    pub device_id: u64,
    pub rx: RxQueue,
    pub tx: TxQueue,
    pub transport: Transport,
    pub mac: [u8; MAC_LEN],
    pub status_supported: bool,
    /// Offset of the link status word in struct virtio_net_config.
    pub net_status_offset: usize,
}

impl Driver {
    /// Stop the device and give it back (plan/50 C2). The reset takes its
    /// queues down; MkDeviceRelease then clears bus mastering and only after
    /// that tears down the DMA and register grants. Nothing is unmapped here
    /// first, so no buffer is freed while the device can still reach it.
    pub fn quiesce(&self) -> bool {
        let stopped = self.transport.reset();
        mk_device_release(self.device_id) >= 0 && stopped
    }
}
