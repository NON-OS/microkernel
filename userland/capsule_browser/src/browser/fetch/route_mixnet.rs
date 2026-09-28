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

use crate::browser::net::{lookup, mixnet};

/// Point the route at the network the reader chose, before a request leaves.
///
/// A reader who chose a private network gets it or gets no page: when its
/// service is not running, the request fails by name. Leaving the route off
/// in that case sent every request direct while the reader believed it was
/// hidden, which is the one disclosure the route exists to prevent. The
/// choice is read here, once per navigation, so a switch takes effect on the
/// next request and never under one already in flight.
pub fn route_mixnet() -> Result<(), &'static str> {
    let net = mixnet::chosen();
    let Some(service) = net.service() else {
        if mixnet::is_on() {
            mixnet::disable();
        }
        return Ok(());
    };
    let port = lookup(service);
    if port == 0 {
        if mixnet::is_on() {
            mixnet::disable();
        }
        return Err(net.absent());
    }
    if mixnet::port() != port {
        mixnet::enable(port);
    }
    Ok(())
}
