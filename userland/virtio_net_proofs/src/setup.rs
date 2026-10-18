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

//! The driver state as the handlers reach it: the shipping struct's fields
//! and its link check. Bring-up and teardown call the broker and are not
//! here; the grant check they make is.

use crate::constants::MAC_LEN;
use crate::queue::{RxQueue, TxQueue};
use crate::transport::Transport;

#[path = "../../capsule_driver_virtio_net/src/setup/grant.rs"]
pub mod grant;
#[path = "../../capsule_driver_virtio_net/src/setup/link.rs"]
mod link;

pub struct Driver {
    pub device_id: u64,
    pub rx: RxQueue,
    pub tx: TxQueue,
    pub transport: Transport,
    pub mac: [u8; MAC_LEN],
    pub status_supported: bool,
    pub net_status_offset: usize,
}
