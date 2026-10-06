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

use smoltcp::wire::{Ipv4Address, Ipv4Cidr};

use crate::state::DNS_SERVERS;

pub struct ConfiguredLease {
    pub address: Ipv4Cidr,
    pub router: Option<Ipv4Address>,
    /// The lease's DNS servers in its order; unused entries are zero.
    pub dns: [[u8; 4]; DNS_SERVERS],
}

pub enum DhcpAction {
    Configured(ConfiguredLease),
    Deconfigured,
    None,
}
