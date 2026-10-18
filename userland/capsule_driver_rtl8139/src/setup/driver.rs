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

//! `Driver` is the live state the server loop holds: the claim, the port
//! grant, the receive buffer and transmit slots over the two DMA grants, and
//! the transmit watchdog. `release` gives the device back, and the broker
//! frees every grant tied to it.

use nonos_libc::Deadline;

use crate::constants::MAC_LEN;
use crate::pio::Pio;

pub struct Driver {
    pub device_id: u64,
    pub claim_epoch: u64,
    pub rx_user_va: u64,
    pub rx_device_addr: u64,
    pub tx_user_va: u64,
    pub tx_device_addr: u64,
    pub rx_offset: usize,
    /// Frames handed to the part, counting up; the slot is `tx_cur % 4`.
    pub tx_cur: usize,
    /// Frames the part has finished with; `tx_cur - tx_dirty` are in flight.
    pub tx_dirty: usize,
    pub pio: Pio,
    pub mac: [u8; MAC_LEN],
    /// Falls due once frames have sat in flight with the link up and no
    /// completion for TX_HANG_MS; renewed on every sign of life.
    pub tx_watch: Deadline,
}

impl Driver {
    /// Stop and reset the part, turn Bus Master off, then give the device
    /// back, which frees every grant (see `rollback`).
    pub fn release(&self) {
        super::rollback::release(self.device_id, self.claim_epoch, &self.pio);
    }
}
