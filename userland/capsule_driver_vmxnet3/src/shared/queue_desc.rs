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

//! The queue descriptor table: one Vmxnet3_TxQueueDesc, then one
//! Vmxnet3_RxQueueDesc, 256 bytes each. Each opens with 16 bytes of ctrl
//! and then its conf, whose offsets are below.

use super::area::Area;
use crate::constants::ring::{RX_COMP, RX_RING, TX_COMP, TX_DATA_DESC, TX_RING};

const CONF: usize = 16;
/// ddPA for a queue: none, as Linux passes.
const NO_DD: u64 = u64::MAX;

/// Every ring's bus address.
pub struct Rings {
    pub tx: u64,
    pub tx_data: u64,
    pub tx_comp: u64,
    pub rx: u64,
    pub rx1: u64,
    pub rx_comp: u64,
}

pub fn write_tx_queue(a: &mut Area, at: usize, r: &Rings) {
    let c = at + CONF;
    a.u64(c, r.tx);
    a.u64(c + 8, r.tx_data);
    a.u64(c + 16, r.tx_comp);
    a.u64(c + 24, NO_DD);
    a.u32(c + 40, TX_RING as u32);
    a.u32(c + 44, TX_RING as u32);
    a.u32(c + 48, TX_COMP as u32);
    a.u16(c + 58, TX_DATA_DESC as u16);
}

pub fn write_rx_queue(a: &mut Area, at: usize, r: &Rings) {
    let c = at + CONF;
    a.u64(c, r.rx);
    a.u64(c + 8, r.rx1);
    a.u64(c + 16, r.rx_comp);
    a.u64(c + 24, NO_DD);
    a.u32(c + 40, RX_RING as u32);
    a.u32(c + 44, RX_RING as u32);
    a.u32(c + 48, RX_COMP as u32);
}
