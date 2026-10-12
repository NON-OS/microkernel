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

//! The address this host scans and connects as. A random one is given to
//! the controller with LE Set Random Address (Vol 4, Part E, 7.8.4), which
//! the controller refuses, Command Disallowed, while it scans or connects
//! with the old one: the caller changes it between those, not during.

use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::addr::{AddrType, BdAddr, RandomKind};
use nonos_bt_wire::hci::opcode as op;
use nonos_bt_wire::WireError;

impl<T: Transport> Host<T> {
    /// Scan and connect as `addr`, a random address. The reserved kind,
    /// top bits 0b10 (Vol 6, Part B, 1.3.2), and a random part 1.3.2.1 or
    /// 1.3.2.2 forbids, are refused unsent.
    pub fn use_random_address(&mut self, addr: BdAddr) -> Result<(), HciError> {
        let kind = addr.random_kind().map_err(HciError::Wire)?;
        if !random_part_allowed(&addr, kind) {
            return Err(HciError::Wire(WireError::Reserved));
        }
        self.command(op::LE_SET_RANDOM_ADDRESS, &addr.0)?;
        self.own = AddrType::Random;
        Ok(())
    }

    /// The address type scans and connections are made as now.
    pub fn own_address(&self) -> AddrType {
        self.own
    }
}

/// Below the two kind bits, a static or non-resolvable address may not be
/// all 0 or all 1, nor may prand's 22 random bits in a resolvable one.
fn random_part_allowed(addr: &BdAddr, kind: RandomKind) -> bool {
    let a = addr.0;
    let bits = match kind {
        RandomKind::Resolvable => u64::from_le_bytes([a[3], a[4], a[5] & 0x3f, 0, 0, 0, 0, 0]),
        _ => u64::from_le_bytes([a[0], a[1], a[2], a[3], a[4], a[5] & 0x3f, 0, 0]),
    };
    let ones = if kind == RandomKind::Resolvable { (1 << 22) - 1 } else { (1 << 46) - 1 };
    bits != 0 && bits != ones
}
