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

//! Where a stream has got to, and where it goes.

use alloc::vec::Vec;

use nonos_route_link::RouteOpening;

use super::link::Link;
use super::socks::Talk;

/// Where a stream goes: a name, never looked up here, or a dotted quad,
/// and a port. Said nowhere: no line names it.
pub struct Dest {
    pub host: Vec<u8>,
    pub port: u16,
}

pub enum Phase {
    /// The SOCKS endpoint, reading the guest's request.
    Asking(Talk),
    /// Waiting for the default network or the service carrying it.
    Route {
        again: i64,
    },
    /// Opening through net.anon's handle front, asked again at `again`.
    Anyone {
        port: u32,
        again: i64,
    },
    /// Opening through net.socks5, on its named stream `stream`.
    Nym {
        opening: RouteOpening,
        stream: u32,
    },
    Open(Link),
    /// It did not open: this errno, until the guest's connect hears it.
    Failed(i64),
    /// The SOCKS endpoint answered a refusal or a failure: end of file
    /// once the guest has read it.
    Closed,
}

impl Phase {
    /// Still opening, on either network or waiting for one.
    pub fn opening(&self) -> bool {
        matches!(self, Phase::Route { .. } | Phase::Anyone { .. } | Phase::Nym { .. })
    }
}
