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

//! Programming both modern virtqueues, receive first, all or nothing.

use nonos_virtio::common::{reset_until, setup_queue, NotifyArea, QueueReady};
use nonos_virtio::{Mmio, VirtioError};

use super::spec::spec;
use crate::clock::expired_after;
use crate::constants::{Q_RX, Q_TX, RESET_MS};

/// Program and enable the receive and transmit queues over the ring
/// regions at device addresses `rx_ring` and `tx_ring`. When either is
/// refused the receive queue may already be enabled, in a grant the failed
/// attempt releases next: the device is reset first, so it has let go of
/// every ring before the grants go.
pub fn program(
    common: &Mmio,
    notify: NotifyArea,
    rx_ring: u64,
    tx_ring: u64,
) -> Result<[QueueReady; 2], &'static str> {
    let both = both(common, notify, rx_ring, tx_ring);
    if both.is_err() {
        let _ = reset_until(common, expired_after(RESET_MS));
    }
    both
}

fn both(
    common: &Mmio,
    notify: NotifyArea,
    rx_ring: u64,
    tx_ring: u64,
) -> Result<[QueueReady; 2], &'static str> {
    let rx_spec = spec(Q_RX, rx_ring).ok_or("virtio-net: rx ring address")?;
    let tx_spec = spec(Q_TX, tx_ring).ok_or("virtio-net: tx ring address")?;
    let rx = setup_queue(common, &rx_spec, notify).map_err(VirtioError::message)?;
    let tx = setup_queue(common, &tx_spec, notify).map_err(VirtioError::message)?;
    Ok([rx, tx])
}
