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

//! A part that resets and also reports both FIFOs drained, the way an idle
//! 8168g or 8125 does: TXCFG_EMPTY in TxConfig, RXTX_EMPTY in MCU, and the
//! 8125B's IntrMitigate bits.

use nonos_devmodel::FakeBar;

use super::model::resetting_part;
use crate::constants::regs::REG_TX_CONFIG;
use crate::hw::regs::TXCFG_EMPTY;
use crate::hw::regs::{INTR_MITIGATE_RXTX_EMPTY, MCU_RXTX_EMPTY, REG_INTR_MITIGATE, REG_MCU};

pub fn draining_part(bar: &FakeBar) {
    resetting_part(bar);
    bar.present32(REG_TX_CONFIG, bar.wrote32(REG_TX_CONFIG) | TXCFG_EMPTY);
    bar.present8(REG_MCU, MCU_RXTX_EMPTY);
    bar.present16(REG_INTR_MITIGATE, INTR_MITIGATE_RXTX_EMPTY);
}
