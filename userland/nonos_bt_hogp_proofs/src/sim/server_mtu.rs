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

//! Exchange MTU (Vol 3, Part F, 3.4.2): the server answers with its own
//! Rx MTU, and the bearer's ATT_MTU becomes the smaller of the two, from
//! the server's side, whatever the client then makes of the answer.

use super::server::{Peripheral, MTU};
use nonos_bt_wire::att::opcode::EXCHANGE_MTU_RSP;

impl Peripheral {
    pub(super) fn exchange(&mut self, p: &[u8]) -> Vec<u8> {
        if let Some(raw) = &self.mtu_reply {
            return [&[EXCHANGE_MTU_RSP][..], raw].concat();
        }
        let client_rx = u16::from_le_bytes([p[0], p[1]]);
        self.mtu = usize::from(client_rx.min(self.rx_mtu)).max(MTU);
        [&[EXCHANGE_MTU_RSP][..], &self.rx_mtu.to_le_bytes()].concat()
    }
}
