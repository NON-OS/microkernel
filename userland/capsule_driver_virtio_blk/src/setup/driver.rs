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

//! `Driver` is the live state the server loop holds: the claim, the queue,
//! how the device is reached, and its size. It holds no DMA grant id: the
//! claim's release takes every grant with it (`quiesce`).

use crate::queue::Queue;
use crate::transport::Transport;

pub struct Driver {
    pub device_id: u64,
    pub claim_epoch: u64,
    pub irq_grant: u64,
    pub queue: Queue,
    pub transport: Transport,
    pub capacity_sectors: u64,
}
