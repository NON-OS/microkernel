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

//! The address this host scans and connects as, and the identity it gives
//! a peer in pairing so the peer can resolve that address later.

use nonos_bt_smp::pairing::Identity;
use nonos_bt_wire::addr::{AddrType, BdAddr};

#[derive(Clone, Copy)]
pub struct Us {
    /// What LE Create Connection used as Own_Address_Type, so what pairing
    /// takes as the initiator's address (Vol 3, Part H, 2.2.7).
    pub kind: AddrType,
    pub addr: BdAddr,
    /// None while this host connects from its public address: there is no
    /// private address for a peer to resolve.
    pub identity: Option<Identity>,
}
