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
use crate::constants::{PORTSC_BASE, PORT_REG_STRIDE};
/// PORTSC of root port `port` (1-based) in the operational registers at
/// `op_base`; `None` for port 0, which names no port.
pub(super) fn portsc_addr(op_base: u64, port: u8) -> Option<u64> {
    let index = (port as u64).checked_sub(1)?;
    Some(op_base + PORTSC_BASE + index * PORT_REG_STRIDE)
}
