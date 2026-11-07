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

//! Who may open a listener on net.core's `net.tcp`: net.sockets alone, and
//! only while the user has chosen the Direct route. Any capsule holding
//! Network can reach `net.tcp`, so this is checked here and not left to
//! net.sockets' own check.

use nonos_policy_proto::route;

/// `sockets_pid` is net.sockets' pid as the service registry gave it;
/// `chosen` is the NetworkRoute the policy store gave.
pub fn listen_allowed(sender_pid: u32, sockets_pid: Option<u32>, chosen: Option<u8>) -> bool {
    sockets_pid.is_some_and(|pid| pid != 0 && pid == sender_pid) && chosen == Some(route::DIRECT)
}

/// Whether listeners already open may stay: only while Direct is chosen.
pub fn listeners_may_stay(chosen: Option<u8>) -> bool {
    chosen == Some(route::DIRECT)
}

/// Whether an ACCEPT may hand out a connection a listener took: only while
/// Direct is still chosen. The reap closes listeners off the Direct route
/// only every two seconds, so ACCEPT asks the route itself before it hands
/// a clearnet connection to anyone.
pub fn accept_allowed(chosen: Option<u8>) -> bool {
    listeners_may_stay(chosen)
}

/// Whether connections a listener took may stay open: only while Direct is
/// chosen. A session accepted on the Direct route does not go on in the
/// clear once the user chooses another route.
pub fn passive_may_stay(chosen: Option<u8>) -> bool {
    listeners_may_stay(chosen)
}
