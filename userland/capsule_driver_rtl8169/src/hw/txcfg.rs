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

use crate::chip::MacVersion;
use crate::constants::regs::{TX_CONFIG_DMA, TX_CONFIG_IFG};

/// TXCFG_AUTO_FIFO (8111e-vl and later 8168s): the TX FIFO is sized by the
/// MAC; the static sizes rtl_set_fifo_size writes are used without it.
const TXCFG_AUTO_FIFO: u32 = 1 << 7;

/// Linux rtl_set_tx_config_registers: unlimited DMA burst, the shortest
/// inter-frame gap, and AUTO_FIFO where rtl_is_8168evl_up holds. The 8125
/// is outside that range in Linux and does not get it.
pub fn tx_config(ver: MacVersion) -> u32 {
    let base = TX_CONFIG_DMA | TX_CONFIG_IFG;
    if ver.is_8168evl_up() {
        base | TXCFG_AUTO_FIFO
    } else {
        base
    }
}
