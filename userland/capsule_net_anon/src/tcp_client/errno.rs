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

//! What went wrong with a call to net.tcp, one value per cause.

pub const E_CALL: u16 = 8;
pub const E_SHORT: u16 = 20;
pub const E_MAGIC: u16 = 21;
pub const E_OP: u16 = 22;
pub const E_LEN: u16 = 23;
pub const E_SHORT_WRITE: u16 = 24;
pub const E_TIMEOUT: u16 = 25;
pub const E_REFUSED: u16 = 26;
/// Added to a service errno so it cannot be confused with the above.
pub const E_ERRNO: u16 = 30;
/// Added to a connection state, so a wait names the state that stopped it.
pub const E_STATE_BASE: u16 = 100;
pub const E_STATE_CALL: u16 = 200;
/// net.tcp reports an empty receive queue as this.
pub const RX_EMPTY: u16 = 11;

/*
 * Two of net.tcp's connect refusals, named here because they mean different
 * things to this capsule. `UNADDRESSABLE` is the stack saying it has no route
 * yet, which before the DHCP lease is every attempt and is worth waiting out
 * rather than retrying at full speed. `LOCAL_PORT_IN_USE` is a defect in the
 * service and waiting will not help.
 */
pub const UNADDRESSABLE: u16 = 14;
pub const LOCAL_PORT_IN_USE: u16 = 15;
