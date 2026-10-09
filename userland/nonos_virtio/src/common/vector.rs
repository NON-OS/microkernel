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

//! MSI-X vector for configuration changes. A queue's vector is set with the
//! queue (`QueueSpec::vector`).

use super::access::CommonCfg;
use super::regs::MSIX_CONFIG;

/// No interrupt for this source. Also what a device reads back when it
/// refuses a vector.
pub const NO_VECTOR: u16 = 0xFFFF;

/// Point configuration-change interrupts at `vector`. True when the device
/// read it back.
pub fn set_config_vector(c: &impl CommonCfg, vector: u16) -> bool {
    c.w16(MSIX_CONFIG, vector);
    c.r16(MSIX_CONFIG) == vector
}
