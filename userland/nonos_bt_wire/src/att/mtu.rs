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

//! Exchange MTU Request (Vol 3, Part F, 3.4.2.1): the largest ATT PDU the
//! client can receive. The bearer's ATT_MTU is then the smaller of this
//! and the server's answer (3.4.2.2).

use super::opcode::EXCHANGE_MTU_REQ;
use crate::cursor::Writer;
use alloc::vec::Vec;

pub fn exchange_mtu(client_rx: u16) -> Vec<u8> {
    let mut w = Writer::new();
    w.u8(EXCHANGE_MTU_REQ).u16(client_rx);
    w.into_vec()
}
