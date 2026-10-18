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

//! The control grant laid out: cleared whole, then the DriverShared area
//! and the queue descriptor table, all before the device is told where it is.

use crate::constants::layout::{
    CONTROL_BYTES, RXQ_DESC_OFF, RX_COMP_OFF, RX_RING1_OFF, RX_RING_OFF, TXQ_DESC_OFF, TX_COMP_OFF,
    TX_DATA_OFF, TX_RING_OFF,
};
use crate::shared::{write_rx_queue, write_shared, write_tx_queue, Area, Rings, SharedConfig};

/// The rings must already be reset: this clears and rewrites only the parts
/// of the grant outside them.
pub fn write(control_va: u64, control_dev: u64) {
    // SAFETY: the control grant is CONTROL_BYTES long, mapped coherent, and
    // the device is not active, so nothing else reads or writes it now.
    let bytes = unsafe { core::slice::from_raw_parts_mut(control_va as *mut u8, CONTROL_BYTES) };
    let mut a = Area::new(&mut bytes[..TX_RING_OFF]);
    a.zero();
    let cfg = SharedConfig {
        queue_desc_pa: control_dev + TXQ_DESC_OFF as u64,
        dd_pa: control_dev,
        dd_len: CONTROL_BYTES as u32,
    };
    write_shared(&mut a, &cfg);
    let at = |off: usize| control_dev + off as u64;
    let rings = Rings {
        tx: at(TX_RING_OFF),
        tx_data: at(TX_DATA_OFF),
        tx_comp: at(TX_COMP_OFF),
        rx: at(RX_RING_OFF),
        rx1: at(RX_RING1_OFF),
        rx_comp: at(RX_COMP_OFF),
    };
    write_tx_queue(&mut a, TXQ_DESC_OFF, &rings);
    write_rx_queue(&mut a, RXQ_DESC_OFF, &rings);
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
}
