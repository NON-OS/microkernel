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

//! What HOSTC should hold. Pure, for the proofs.

use crate::regs::{HOSTC_HST_EN, HOSTC_I2C_EN};

/// The host enabled and SMBus (not I2C) timing; every other bit, SMI routing
/// among them, as firmware set it. The broker lets exactly these two bits
/// change (`src/hardware/broker/pci/quirk_bits.rs`).
pub fn hostc_wanted(current: u16) -> u16 {
    (current & !HOSTC_I2C_EN) | HOSTC_HST_EN
}
